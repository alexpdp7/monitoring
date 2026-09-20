#[derive(serde::Serialize, serde::Deserialize)]
pub enum SubCheckStatus {
    OK,
    WARNING,
    CRITICAL,
    UNKNOWN,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SubCheckResult {
    pub id: String,
    pub status: SubCheckStatus,
    pub description: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CheckResult {
    pub subchecks: Vec<SubCheckResult>,
}
