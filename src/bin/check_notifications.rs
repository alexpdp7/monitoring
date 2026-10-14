pub fn main() {
    let quack_token = std::env::var("MONITORING_QUACK_TOKEN")
        .expect("MONITORING_QUACK_TOKEN environment variable to be set");
    let connection = duckdb::Connection::open_in_memory().unwrap();
    connection
        .execute(
            "ATTACH 'quack:localhost' as remote (TOKEN ?);",
            duckdb::params![quack_token],
        )
        .unwrap();
    let mut statement = connection
        .prepare("select check_id, subcheck_id, status, inserted_at, description from remote.last_check_results")
        .unwrap();

    let mut last_check_results = statement.query([]).unwrap();

    while let Some(row) = last_check_results.next().unwrap() {
        eprintln!(
            "{} {} {} {} {}",
            row.get_unwrap::<usize, String>(0),
            row.get_unwrap::<usize, String>(1),
            row.get_unwrap::<usize, String>(2),
            row.get_unwrap::<usize, chrono::NaiveDateTime>(3),
            row.get_unwrap::<usize, String>(4),
        );
    }
}
