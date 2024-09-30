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
  

  // Example:
  CREATE OR REPLACE FUNCTION ghost_teacher() RETURNS teacher AS $$
    SELECT 22 AS ID, 'ghost_name' AS name, 200::DECIMAL AS salary;
  $$ LANGUAGE SQL;
  

  // Example
  CREATE OR REPLACE FUNCTION ghost_teacher() RETURNS teacher AS $$
    SELECT 22, 'ghost_name', 200::DECIMAL;
  $$ LANGUAGE SQL;

  
  SELECT id, salary 
  FROM ghost_teacher();

```

- SETOF

```

  CREATE TABLE teacher (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    salary DECIMAL(10, 2)
  );


  INSERT INTO teacher(name, salary) VALUES('Roberto', 100);
  INSERT INTO teacher(name, salary) VALUES('Carol', 200);
  INSERT INTO teacher(name, salary) VALUES('Pedro', 300);
  INSERT INTO teacher(name, salary) VALUES('Carla', 400);
  INSERT INTO teacher(name, salary) VALUES('Nico', 500);

  CREATE FUNCTION pay_teachers(value_salary DECIMAL) RETURNS SETOF teacher AS $$
    SELECT * FROM teacher WHERE salary > value_salary;
  $$ LANGUAGE SQL;

  SELECT * FROM pay_teachers(200);


  // OBS
  CREATE FUNCTION pay_teachers(value_salary DECIMAL) RETURNS teacher AS $$
    SELECT * FROM teacher WHERE salary > value_salary;
  $$ LANGUAGE SQL;

  SELECT * FROM pay_teachers(300);

  // Output = Carla, 400
  
  
```

- RETURN

```
    
  CREATE FUNCTION pay_teachers(value_salary DECIMAL, OUT name VARCHAR, OUT salary DECIMAL) RETURNS SETOF RECORD AS $$
    SELECT name, salary FROM teacher WHERE salary > value_salary;
  $$ LANGUAGE SQL;

  SELECT * FROM pay_teachers(200);

```

- PLSQL

```
  CREATE FUNCTION first_pl() RETURNS INTEGER AS $$
    BEGIN 
      RETURN 1;
    END
  $$ LANGUAGE PLSQL; // plsql

  SELECT first_pl;
  
```

- Variables

```
  CREATE FUNCTION first_pl() RETURNS INTEGER AS $$
    DECLARE
      first_variable INTERGER DEFAULT 3; // := 3
    BEGIN 
      first_variable := first_variable * 2;
      RETURN first_variable;
    END
  $$ LANGUAGE PLSQL; // plsql

  SELECT first_pl;
  
```

- Variables - Child

```
  CREATE FUNCTION first_pl() RETURNS INTEGER AS $$
    DECLARE
      first_variable INTERGER DEFAULT 3; // := 3
    BEGIN 
      first_variable := first_variable * 2;
         
      BEGIN
       first_variable := 8; 
      END;

      RETURN first_variable;
     END
  $$ LANGUAGE PLSQL; // plsql

  SELECT first_pl;
  
```

- ELSE and IF

```
  CREATE FUNCTION salary_ok(teacher teacher) RETURNS VARCHAR AS $$
    BEGIN 
      IF teacher.salary > 200 THEN
        RETURN 'Salary - OK'
      ELSE 
        RETURN 'Selary add Plus!'
      END IF;
    END;
  $$ LANGUAGE plsql;

  SELECT name, salary_ok,(teacher)
  FROM teacher;
  
```

- ELSEIF

```
  CREATE FUNCTION salary_ok(teacher teacher) RETURNS VARCHAR AS $$
    BEGIN 
      
      IF teacher.salary > 200 THEN
        RETURN 'Salary - OK'
      
      ELSEIF teacher.salary = 30 THEN
        RETURN 'Add Plus!'
      
      ELSE 
        RETURN 'Urgent Salary Plus!'
      END IF;
    
    END;
  $$ LANGUAGE plsql;

  SELECT name, salary_ok,(teacher)
  FROM teacher;
  
```

- CASE

```
   CREATE FUNCTION salary_ok(teacher teacher) RETURNS VARCHAR AS $$
    BEGIN 
        
      CASE 
        WHEN teacher.salary = 100 THEN
          RETURN 'Low salary';
          
        WHEN teacher.salary = 200 THEN 
          RETURN 'Mediun salary';

        WHEN teacher.salary = 300 THEN
          RETURN 'Salary Ok!';
        
        ELSE
          RETURN 'Salary Good!'; 

      END CASE;
    END;
  $$ LANGUAGE plsql;

  SELECT name, salary_ok,(teacher)
  FROM teacher;
  

  // Resume
  CREATE FUNCTION salary_ok(teacher teacher) RETURNS VARCHAR AS $$
    BEGIN 
        
      CASE teacher.salary
        WHEN 100 THEN
          RETURN 'Low salary';
          
        WHEN 200 THEN 
          RETURN 'Mediun salary';

        WHEN 300 THEN
          RETURN 'Salary Ok!';
        
        ELSE
          RETURN 'Salary Good!'; 

      END CASE;
    END;
  $$ LANGUAGE plsql;

  SELECT name, salary_ok,(teacher)
  FROM teacher;
  
```
