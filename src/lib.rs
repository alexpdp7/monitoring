//! `monitoring` is a Nagios-inspired minimal monitoring system.
//!
//! `monitoring` is composed of the following executables:
//!
//! * [`executor`](../executor/index.html): runs monitoring checks based on a configuration.
//! * [`duck_storage`](../duck_storage/index.html): stores monitoring check results in a DuckDB database.
//!
//! The crate also includes the following monitoring check commands:
//!
//! * [`check_url`](../check_url/index.html)
//! * [`check_ragent`](../check_ragent/index.html): a check for the <https://github.com/alexpdp7/ragent> monitoring agent

use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
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

impl FullCheckResult {
    pub fn summary(&self) -> HashMap<SubCheckStatus, i32> {
        let (ok, warning, critical, unknown) =
            self.result.subchecks.iter().fold((0, 0, 0, 0), |c, sc| {
                (
                    c.0 + if sc.status == SubCheckStatus::OK {
                        1
                    } else {
                        0
                    },
                    c.1 + if sc.status == SubCheckStatus::WARNING {
                        1
                    } else {
                        0
                    },
                    c.2 + if sc.status == SubCheckStatus::CRITICAL {
                        1
                    } else {
                        0
                    },
                    c.3 + if sc.status == SubCheckStatus::UNKNOWN {
                        1
                    } else {
                        0
                    },
                )
            });
        HashMap::from([
            (SubCheckStatus::OK, ok),
            (SubCheckStatus::WARNING, warning),
            (SubCheckStatus::CRITICAL, critical),
            (SubCheckStatus::UNKNOWN, unknown),
        ])
    }
}
