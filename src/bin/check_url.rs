use std::io::Write;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "expected a single argument");
    let url = &args[1];
    let status = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .build()
        .new_agent()
        .get(url)
        .call()
        .expect("url call not to fail")
        .status();
    std::io::stderr()
        .write(
            serde_json::to_string(&monitoring::CheckResult {
                subchecks: vec![monitoring::SubCheckResult {
                    id: "request".into(),
                    status: if status.is_success() {
                        monitoring::SubCheckStatus::OK
                    } else {
                        monitoring::SubCheckStatus::CRITICAL
                    },
                    description: format!("request to {url} returned {status}"),
                }],
            })
            .unwrap()
            .as_bytes(),
        )
        .unwrap();
}
