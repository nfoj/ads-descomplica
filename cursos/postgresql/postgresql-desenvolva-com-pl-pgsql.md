#  PostgreSQL: desenvolva com PL/pgSQL

- Basic

```
  CREATE FUNCTION fn_name() RETURNS void AS '
    DELETE FROM emp
    WHERE salary < 0;
  ' LANGUAGE SQL;



  // Example
  CREATE FUNCTION fn_sum() RETURNS INTEGER AS '
    SELECT (5 + 3) * 2;
  'LANGUAGE SQL;

  SELECT fn_sum();
  
  SELECT fn_sum() AS rename;

```

- Function Sum

```
  CREATE FUNCTION sum_two (num1 INTEGER, num2 INTEGER) RETURNS INTEGER AS '
    SELECT num1 + num2;
  'LANGUAGE SQL;

  SELECT sum_two(2,2);

```

- Replace and Void

```
  // With replace, drop no used

  CREATE TABLE name_table (name VARCHAR(255) NOT NULL);

  // With void
  CREATE OR REPLACE FUNCTION create_name_table(name VARCHAR) RETURNS void AS '
    INSERT INTO name_table(name) VALUES(create_name_table.name);
  ' LANGUAGE SQL;

  SELECT create_name_table('Rodolfo');

  // Not void = ADD SELECT + 
  CREATE OR REPLACE FUNCTION create_name_table(name VARCHAR) RETURNS VARCHAR AS '
    INSERT INTO name_table(name) VALUES(create_name_table.name);
    SELECT name; 
 'LANGUAGE SQL;
  
```

- $$ and $$ = ''

```
  CREATE OR REPLACE FUNCTION create_name_table(name VARCHAR) RETURNS void AS $$
    INSERT INTO name_table(name) VALUES('Roberto');
  $$ LANGUAGE SQL;
  
```

- Multiplication

```
  CREATE TABLE teacher (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    salary DECIMAL(10, 2)
  );


  INSERT INTO teacher(name, salary) VALUES('Roberto', 100);


  CREATE FUNCTION double_salary(teacher) RETURNS DECIMAL AS $$
    SELECT $1.salary * 2 AS salary_double;
  $$ LANGUAGE SQL;
  

  SELECT name, double_salary(teacher.*) 
  FROM teacher;
  
```


