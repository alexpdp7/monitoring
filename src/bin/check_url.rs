use std::io::Write;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "expected a single argument");
    let url = &args[1];
    let (status, description) = match ureq::Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .new_agent()
        .get(url)
        .call()
    {
        Ok(response) => (
            {
                if response.status().is_success() {
                    monitoring::SubCheckStatus::OK
                } else {
                    monitoring::SubCheckStatus::CRITICAL
                }
            },
            format!("request to {url} returned {}", response.status()),
        ),
        Err(err) => (
            monitoring::SubCheckStatus::CRITICAL,
            format!("error {err} requesting {url}"),
        ),
    };
    std::io::stdout()
        .write(
            serde_json::to_string(&monitoring::CheckResult {
                subchecks: vec![monitoring::SubCheckResult {
                    id: "request".into(),
                    status,
                    description,
                }],
            })
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
}
