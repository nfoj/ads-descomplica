#  PostgreSQL: desenvolva com PL/pgSQL

- Basic

```
  CREATE FUNCTION fn_name() RETURNS void AS '
    DELETE FROM emp
    WHERE salary < 0;
  ' LANGUAGE SQL;



  // Example
  CREATE FUNCTION fn_sum() RETURNS INTERGER AS '
    SELECT (5 + 3) * 2
  'LANGUAGE SQL;

  SELECT fn_sum();
  
  SELECT fn_sum() AS rename;
  

  
```

