pub fn main() {
    let connection = duckdb::Connection::open("foo").unwrap();
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
