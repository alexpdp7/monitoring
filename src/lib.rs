#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum SubCheckStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
