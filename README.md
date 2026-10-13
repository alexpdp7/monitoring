# monitoring

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
