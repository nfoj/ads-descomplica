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


- Create table

```
  DROP TABLE instrutor;
  CREATE TABLE instrutor (
      id SERIAL PRIMARY KEY,
      nome VARCHAR(255) NOT NULL,
      salario DECIMAL(10, 2)
  );
  SELECT COUNT(*) FROM instrutor;
  DO $$
      DECLARE
      BEGIN
          FOR i IN 1..1000000 LOOP
              INSERT INTO instrutor (nome, salario) VALUES ('Instrutor(a) ' || i, random() * 1000 + 1);
          END LOOP;
      END;
  $$;
  UPDATE instrutor SET salario = salario * 2 WHERE id % 2 = 1;
  DELETE FROM instrutor WHERE id % 2 = 0;
  VACUUM ANALYSE instrutor;

  SELECT relname, n_dead_tup FROM pg_stat_user_tables;
  SELECT pg_size_pretty(pg_relation_size('instrutor'));

  
```

- VACUUM

```
  // Optimize
  VACUUM VERBOSE;

  // Optimize + Organization
  VACCUM FULL;

  // Table update status
  VACCUM ANALIZE;
  
```


- Analyze + Reindex

```
  ANALYZE;
  REINDEX TABLE student;

  
```

- Backup = dump

```

  // terminal
  pg dump -f /tmp/dum.sql student

  
  // pgadmin
  tools > backup
    
```

- Import

```
  // pgadmin 
  tools > backup > restore


  //terminal
  pg restore -d student /tmp/dump.sql // tar

  psql student < /tmp/dump.sql // all formats
  
```

- Planning

```
  EXPLAIN SELECT * FROM instrutor WHERE salario > 1500;

  CREATE INDEX idx_salario ON instrutor(salario);
  
```

- pg_hba = file acess

```
  // acess
  /usr/local/pgsql/data 

  // vim, nvim, nano, helix ...
  vim pg_hba.conf

  local     DATABASE   USER             METHOD   [OPTIONS]
  host      DATABASE   USER   ADDRESS   METHOD   [OPTIONS]
  hostssl   DATABASE   USER   ADDRESS   METHOD   [OPTIONS]
  hostnossl DATABASE   USER   ADDRESS   METHOD   [OPTIONS]


  //Example - insert

  local   DATABASE   user        ADDRESS          METHOD (password, md5, trust)
  host    all        all         all              password
  host    student    postgress   127.0.0.2/32     trust
 
```

- roles

```
  CREATE USER name;

  // access
  /usr/local/pgsql/data 
  vim pg_hba.conf
  host student name 127..0.0.2/32 trust
  pg_ctl restart


  DROP ROLE name; 
  CREATE USER name password '12345';
  
  // access
  /usr/local/pgsql/data 
  vim pg_hba.conf
  host student name 127..0.0.2/32 password
  pg_ctl restart


  // SUPERUSER
  CREATE USER name password '123' NOSUPERUSER CREATEDB;


  // REVOKE 
  REVOKE ALL ON DATABASE student FROM name;
  GRANT SELECT ON public.student TO name; 
  
```
