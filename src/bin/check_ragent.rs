//! A check for the <https://github.com/alexpdp7/ragent> monitoring agent.

use monitoring::{SubCheckResult, SubCheckStatus};
use ragent::RagentInfo;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 2, "expected a single argument");
    let uri = format!("http://{}:21488", &args[1]);
    let request = ureq::get(uri.clone()).call();
    let subchecks = match request {
        Ok(mut request) => as_subchecks(request.body_mut().read_json::<RagentInfo>()),
        Err(err) => {
            vec![monitoring::SubCheckResult {
                id: "request".into(),
                status: monitoring::SubCheckStatus::UNKNOWN,
                description: format!("error getting {uri}: {err}"),
            }]
        }
    };
    println!(
        "{}",
        serde_json::to_string(&monitoring::CheckResult { subchecks }).unwrap()
    );
}

fn as_subchecks(ragent_info: Result<RagentInfo, ureq::Error>) -> Vec<monitoring::SubCheckResult> {
    match ragent_info {
        Err(err) => vec![monitoring::SubCheckResult {
            id: "request".into(),
            status: monitoring::SubCheckStatus::UNKNOWN,
            description: format!("error parsing json: {err}"),
        }],
        Ok(ragent_info) => [
            filesystems_as_subchecks(ragent_info.filesystems),
            units_as_subchecks(ragent_info.units),
            reboot_as_subchecks(ragent_info.reboot),
            entropy_as_subchecks(ragent_info.entropy_available, ragent_info.entropy_pool_size),
        ]
        .concat(),
    }
}

fn entropy_as_subchecks(entropy_available: usize, entropy_pool_size: usize) -> Vec<SubCheckResult> {
    let warning = entropy_available / 2;
    let critical = entropy_available / 4;

    vec![if entropy_available > warning {
        SubCheckResult {
            id: "entropy".into(),
            status: SubCheckStatus::OK,
            description: format!("entropy {entropy_available} more than half {entropy_pool_size}"),
        }
    } else if entropy_available > critical {
        SubCheckResult {
            id: "entropy".into(),
            status: SubCheckStatus::WARNING,
            description: format!("entropy {entropy_available} less than half {entropy_pool_size}"),
        }
    } else {
        SubCheckResult {
            id: "entropy".into(),
            status: SubCheckStatus::CRITICAL,
            description: format!(
                "entropy {entropy_available} less than a quarter {entropy_pool_size}"
            ),
        }
    }]
}

fn reboot_as_subchecks(reboot: ragent::reboot::Reboot) -> Vec<SubCheckResult> {
    vec![match reboot.reboot_required {
        true => SubCheckResult {
            id: "reboot".into(),
            status: SubCheckStatus::WARNING,
            description: "reboot required".into(),
        },
        false => SubCheckResult {
            id: "reboot".into(),
            status: SubCheckStatus::OK,
            description: "reboot not required".into(),
        },
    }]
}

fn units_as_subchecks(units: Vec<ragent::systemd::Unit>) -> Vec<SubCheckResult> {
    units.into_iter().map(unit_as_subcheck).collect::<Vec<_>>()
}

fn unit_as_subcheck(unit: ragent::systemd::Unit) -> SubCheckResult {
    SubCheckResult {
        id: format!("systemd-unit-{}", unit.id),
        status: match unit.active_state.as_str() {
            "failed" => SubCheckStatus::CRITICAL,
            _ => SubCheckStatus::OK,
        },
        description: format!("systemd unit {} is {}", unit.id, unit.active_state),
    }
}

fn filesystems_as_subchecks(
    filesystems: Vec<ragent::filesystems::Filesystem>,
) -> Vec<SubCheckResult> {
    filesystems
        .into_iter()
        .flat_map(filesystem_as_subchecks)
        .collect::<Vec<_>>()
}

fn filesystem_as_subchecks(filesystem: ragent::filesystems::Filesystem) -> Vec<SubCheckResult> {
    filesystem_as_space_subcheck(&filesystem)
        .into_iter()
        .chain(filesystem_as_inode_subcheck(&filesystem).into_iter())
        .collect::<Vec<_>>()
}

fn filesystem_as_space_subcheck(
    filesystem: &ragent::filesystems::Filesystem,
) -> Option<SubCheckResult> {
    if filesystem.size_bytes == 0 {
        return None;
    }
    let warning = std::cmp::min(filesystem.size_bytes / 5, 2 * 1024 * 1024 * 1024);
    let critical = std::cmp::min(filesystem.size_bytes / 10, 1024 * 1024 * 1024);
    let (status, description) = check_thresholds(filesystem.available_bytes, warning, critical);
    Some(SubCheckResult {
        id: format!("filesystem-space-{}", filesystem.mount_point),
        status,
        description,
    })
}

fn filesystem_as_inode_subcheck(
    filesystem: &ragent::filesystems::Filesystem,
) -> Option<SubCheckResult> {
    if filesystem.available_inodes == 0 {
        return None;
    }
    let warning = filesystem.inodes / 5;
    let critical = filesystem.inodes / 10;
    let (status, description) = check_thresholds(filesystem.available_inodes, warning, critical);
    Some(SubCheckResult {
        id: format!("filesystem-inodes-{}", filesystem.mount_point),
        status,
        description,
    })
}

fn check_thresholds(value: u64, warning: u64, critical: u64) -> (SubCheckStatus, String) {
    if value > warning {
        (
            SubCheckStatus::OK,
            format!("{value} is over warning threshold {warning}"),
        )
    } else if value > critical {
        (SubCheckStatus::WARNING, format!("{value} is below warning threshold {warning} but over critical threshold {critical}"))
    } else {
        (
            SubCheckStatus::CRITICAL,
            format!("{value} is below critical threshold {critical}"),
        )
    }
}
