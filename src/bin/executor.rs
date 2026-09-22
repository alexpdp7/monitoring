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
                println!(
                    "{}",
                    serde_json::to_string(&monitoring::FullCheckResult {
                        id: check.id.clone(),
                        result: check.check().await,
                    })
                    .unwrap(),
                );
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

    pub async fn check(&self) -> monitoring::CheckResult {
        let execution = tokio::process::Command::new(self.command[0].clone())
            .args(&self.command[1..])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap_or_else(|_| panic!("could not spawn {self:?}"))
            .wait_with_output()
            .await
            .unwrap();
        if execution.status.success() {
            serde_json::from_slice::<monitoring::CheckResult>(&execution.stdout).unwrap_or_else(
                |err| monitoring::CheckResult {
                    subchecks: vec![monitoring::SubCheckResult {
                        id: "check".into(),
                        status: monitoring::SubCheckStatus::UNKNOWN,
                        description: format!("parsing error {err}"),
                    }],
                },
            )
        } else {
            monitoring::CheckResult {
                subchecks: vec![monitoring::SubCheckResult {
                    id: "check".into(),
                    status: monitoring::SubCheckStatus::UNKNOWN,
                    description: format!("return code {}", execution.status),
                }],
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct Configuration {
    checks: Vec<Check>,
}
