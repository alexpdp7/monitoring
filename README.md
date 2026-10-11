# monitoring

```console
$ ./sample-run
```

On a separate terminal:

```console
$ duckdb
FROM quack_query('quack:localhost', 'select * from check_results', token = 'sample-token');
```

[Design notes](https://alex.corcoles.net/notes/tech/a-monitoring-system)
