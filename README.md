# monitoring

```console
$ ./sample-run
```

On a separate terminal:

```console
$ duckdb
FROM quack_query('quack:localhost', 'select * from check_results', token = 'sample-token');
```

You can also view the DuckDB user interface at <http://localhost:4213/>.

[Design notes](https://alex.corcoles.net/notes/tech/a-monitoring-system)
