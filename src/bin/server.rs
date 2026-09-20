#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "expected a single argument");
    let configuration = &args[1];
    let configuration: Configuration = serde_json::from_reader(
        std::fs::File::open(configuration)
            .unwrap_or_else(|_| panic!("Could not open {configuration}")),
    )
    .unwrap();
    eprintln!("{:?}", configuration);
    let mut check_tasks = tokio::task::JoinSet::new();
    for check in configuration.checks {
        check_tasks.spawn(async move {
            tokio::time::sleep(check.initial_delay()).await;
            let mut interval = tokio::time::interval(check.period());
            loop {
                interval.tick().await;
                check.check().await;
            }
        });
    }
    check_tasks.join_next().await;
    panic!("some check task exited!");
}

#[derive(Debug, serde::Deserialize)]
struct Check {
    id: String,
    command: Vec<String>,
    period_seconds: u64,
}

impl Check {
    pub fn period(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.period_seconds)
    }

    pub fn initial_delay(&self) -> std::time::Duration {
        rand::random_range(std::time::Duration::ZERO..self.period())
    }

    pub async fn check(&self) {
        tokio::process::Command::new(self.command[0].clone())
            .args(&self.command[1..])
            .spawn()
            .unwrap_or_else(|_| panic!("could not spawn {self:?}"))
            .wait()
            .await
            .unwrap();
    }
}

#[derive(Debug, serde::Deserialize)]
struct Configuration {
    checks: Vec<Check>,
}
