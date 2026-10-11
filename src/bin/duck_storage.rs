pub fn main() {
    let duckdb_path = std::env::var("MONITORING_DUCKDB_PATH")
        .expect("MONITORING_DUCKDB_PATH environment variable to be set");
    let quack_token = std::env::var("MONITORING_QUACK_TOKEN")
        .expect("MONITORING_QUACK_TOKEN environment variable to be set");
    let connection = duckdb::Connection::open(duckdb_path).unwrap();
    connection
        .execute_batch(
            r#"
      create or replace type check_status as enum('OK', 'WARNING', 'CRITICAL', 'UNKNOWN');
      create or replace table check_results (
        check_id           varchar not null,
        subcheck_id        varchar not null,
        status             check_status,
        description        varchar,
        inserted_at        timestamp not null default now()
      );
    "#,
        )
        .unwrap();

    let (listen_uri, http_url, token) = connection
        .query_row(
            "call quack_serve('quack:localhost', token = ?);",
            duckdb::params![quack_token],
            |row| {
                Ok((
                    row.get::<usize, String>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, String>(2).unwrap(),
                ))
            },
        )
        .unwrap();

    eprintln!("duck db listen_uri {listen_uri} http_url {http_url} token {token}");

    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("failed reading from stdin");
        let full_check_result =
            serde_json::from_str::<monitoring::FullCheckResult>(&buffer).expect("parsing error");
        eprintln!("{:?}", full_check_result);
        for subcheck in full_check_result.result.subchecks {
            connection.execute("insert into check_results(check_id, subcheck_id, status, description) values (?, ?, ?, ?)", duckdb::params![full_check_result.id, subcheck.id, format!("{:?}", subcheck.status), subcheck.description]).unwrap();
        }
    }
}
