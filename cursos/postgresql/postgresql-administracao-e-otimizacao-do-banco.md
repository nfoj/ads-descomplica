# PostgreSQL: administração e otimização do banco

- Creating a Database Cluster

```
  initdb -D /usr/local/pgsql/data
  
```

- Starting the Database Server

```
  postgres -D /usr/local/pgsql/data
  
```

- pg_ctl

```

  pg_ctl start

  pg_ctl status

  pg_ctl restart

  pg_ctl stop
  
  ...
  
```

- Postgres.conf

```
  cd /usr/local/pgsql/data

  ls -l 

  more postgresql.conf

  more postgresql.conf | grep -v '#' | grep -eve '^$'
  
```

- Customized Options

```

  // vim // nvim // helix // nano ...
  vim postgresql.conf

  log_connections=yes
  log_destinatiion=stderr

  // shared_buffer - B, KB, MB, GB and TB
  shared_buffers=4GB

  
```

- Obs: shard_buffers - Cache memory allocation: 15% - 25% of total available RAM.




- Init

``` 
// Init
  pg_ctl -D /usr/local/pgsql/data -l /tmp/pg.log start

  pg_ctl -D /usr/local/pgsql/data start

  pg_ctl status

```
