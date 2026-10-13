# monitoring

> [!CAUTION]
> [`.cargo/config.toml`](.cargo/config.toml) [sets the `DUCKDB_DOWNLOAD_LIB` environment variable](https://github.com/duckdb/duckdb-rs#notes-on-building-duckdb-and-libduckdb-sys).
> This variable makes the build download a DuckDB binary from GitHub releases.
> Downloading binaries adds extra supply chain risks to the build process, in addition to any other Cargo build risk from all dependencies.
>
> Without downloading binaries, the build process consumes a significant amount of extra resources.
>
> (My understanding of the duckdb-rs documentation is that the download should happen by default, but apparently it does not.)

```console
$ ./sample-run
```

On a separate terminal:

```console
$ duckdb
FROM quack_query('quack:localhost', 'FROM quack_query('quack:localhost', 'select last_check_results.*, last_ok_check_results.inserted_at as last_ok_inserted_at from last_check_results left join last_ok_check_results using (check_id, subcheck_id)', token = 'sample-token');
```

You can also view the DuckDB user interface at <http://localhost:4213/>.

[Design notes](https://alex.corcoles.net/notes/tech/a-monitoring-system)
