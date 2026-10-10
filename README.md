# monitoring

```console
$ ./sample-run
...
duck db listen_uri quack:localhost http_url http://localhost:9494 token <TOKEN>
...
```

On a separate terminal:

```console
$ duckdb
FROM quack_query('quack:localhost', 'select * from check_results', token = '<TOKEN>');
```

[Design notes](https://alex.corcoles.net/notes/tech/a-monitoring-system)
