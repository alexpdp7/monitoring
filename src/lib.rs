//! `monitoring` is a Nagios-inspired minimal monitoring system.
//!
//! `monitoring` is composed of the following executables:
//!
//! * [`executor`](../executor/index.html): runs monitoring checks based on a configuration.
//! * [`duck_storage`](../check_url/index.html): stores monitoring check results in a DuckDB database.

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum SubCheckStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SubCheckResult {
    pub id: String,
    pub status: SubCheckStatus,
    pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CheckResult {
    pub subchecks: Vec<SubCheckResult>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FullCheckResult {
    pub id: String,
    pub result: CheckResult,
}
