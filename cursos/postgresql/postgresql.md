# PostgreSQL


- Basic Commands 

```
  \h = help!

  \h create role = help create role

  \? = list commands

  \l = list data base

  \du = list users

  \c <data base> = acess data base especification

  \q = exit

  \d = list table
  \dS = list table system 

  \! = terminal
    exit > user sql

  \! date = acess terminal command
  \! pwd
  
```


- Create DBase

```
  // #
  CREATE DATABASE "name" 
  WITH
  OWNER = postgres
  ENCODING = 'UTF8'
  CONNECTION LIMIT = -1;
  
```

  -1: no limit connect


- Delete DBase

```
  DROP DATABASE "name";
  
```


- Types

  - Numerics:
    integer (-1 ... 1)
    real (-1.0 ... 1.0)
    serial (+1.0 - Autoincrement)
    numeric (decimal types explict - .00, .000)  

  - String:
    varchar (n) 
    char (n definition)
    text (no definition)

  - Boolean
    true
    false

  - time
    date 'YYYY-MM-DD'
    time 'HH24:MM:SS'
    timestamp 'YYYY-MM-DD HH24:MM:SS'


- Types - Use

```

  CREATE TABLE student (
    id SERIAL,
    name VARCHAR(255),
    cpf CHAR(11),
    observation TEXT,
    year INTEGER,
    money NUMERIC(10,2),
    height real,
    active BOOLEAN
    date_of_birth DATE
    hour TIME,
    registration TIMESTAMP        
  );

  
  // NUMERIC(10,2): 12345678,90

```


- Insert sigle row into table

```
  INSERT INTO "table name" (column_1, column2, column_3, ...)
    VALUES('value_column_1', 'value_column_2',value_column_3, ...);


  INSERT INTO student (name, cpf, observation, year, money, height, active, date_of_birth, hour, registration)
    VALUES ('Alice', '00000000000', 'text', 29, 100.19, 1.69, true, '1999-10-11', '17:48:00', '2023-06-02 08:40:00');

  
```


- CRUD (Create, Read, Update and Delete)

  SELECT: column or * (all columns)
    FROM: table name
    WHERE: number (id - serial);

  UPDATE: name table
    SET column = 'new value'
    WHERE: number (id - serial);

  DELETE: column or * (all columns)
    FROM: table name
    WHERE: name 'name'
    
```
  SELECT *
    FROM student
    WHERE id = 1;

  UPDATE student
    SET name = 'Roberto', 
    cpf = '11111111111',
    observation = 'Text 2',
    year = 50, 
    money = 10.19,
    height = 2.01,
    active = FALSE,
    date_of_birth = '1990-01-19'
    hour = '11:20:00',
    registration = '2022-11-14 11:13:00'
    WHERE id = 1;


  SELECT *
    FROM student
    WHERE name = 'Roberto';

  DELETE *
    FROM student
    WHERE name = 'Roberto';  
  
```


- SELECT and AS

  SELECT: column or * (all columns)
  AS: temporary names

```
  SELECT name, year, ...
    FROM student;

  SELECT name AS "FIRST and LAST",
    year
    observation AS obs
  FROM student;  
  
```


- Filters

```

  // equal: =
  SELECT *
  FROM student
  WHERE name = 'Alice'

  // not equal to: != or <>
  SELECT * 
  FROM student
  WHERE name != 'Alice'
  
  // similar: LIKE + _ or % 
  SELECT *
  FROM student
  WHERE name LIKE '_lice'

  // not similar: NOT LIKE + _ or %
  SELECT *
  FROM student
  WHERE name NOT LIKE 'Alic_'

  // IS NULL = empty
  SELECT * 
  FROM student
  WHERE cpf IS NULL 

  // IS NOT NULL = no empty
  SELECT *
  FROM student
  WHERE cpf IS NOT NULL

```


- Example: 

```

  INSERT INTO student (name) VALUES ('Alice');
  INSERT INTO student (name) VALUES ('Aline');
  INSERT INTO student (name) VALUES ('Ana');
  INSERT INTO student (name) VALUES ('Alana');
  INSERT INTO student (name) VALUES ('Alcides');
  INSERT INTO student (name) VALUES ('Alceudes');


  SELECT * 
  FROM student
  WHERE name = 'Alice'

  // Output
  Alice


  SELECT *
  FROM student
  WHERE name = 'Joao'

  // Output
  empty


  SELECT *  
  FROM student
  WHERE name != 'Alice'

  // Output
  Aline
  Ana
  Alana
  Alcides
  Alceudes


  SELECT * 
  FROM student
  WHERE name LIKE '_lice'

  // Output
  Alice
  Aline


  SELECT *
  FROM student
  WHERE name NOT LIKE 'Ana'

  // Output
  Alice
  Aline
  Alana
  Alcides
  Alceudes


  SELECT *
  FROM student
  WHERE name LIKE 'Al%'

  // Output
  Alice
  Aline
  Alana
  Alcides
  Alceudes


  SELECT * 
  FROM student
  WHERE name LIKE '%c%'

  // Output
  Alcides
  Alceudes
  
```


- Num: =, !=, <, >, <=, >=, BETWEEN 

```

  SELECT *
  FROM student
  WHERE year = 10

  SELECT *
  FROM student
  WHERE year != 10

  SELECT *
  FROM student
  WHERE year < 10

  SELECT *
  FROM student
  WHERE year > 10

  SELECT *
  FROM student
  WHERE year <= 10


  SELECT *
  FROM student
  WHERE year >= 10


  SELECT *
  FROM student
  WHERE year BETWEEN 10 AND 20

```


- Bool: true or false 

```

  SELECT *
  FROM student
  WHERE active = false

  SELECT * 
  FROM student
  WHERE active = true
  
```


- AND - OR

```

  SELECT * 
  FROM student
  WHERE name LIKE 'A%'
  AND cpf IS NOT NULL


  SELECT * 
  FROM student
  WHERE name LIKE 'Alice'
  OR name LIKE 'Sebastian'
  OR name LIKE 'Ana'
  
```

- Primary Key

```
  CREATE TABLE test;
    id INTEGER PRIMARY KEY;
    name VARCHAR(255) NOT NULL;


  INSERT INTO test (id, name) VALUES (1, 'HTML');
  INSERT INTO test (id, name) VALUES (2, 'CSS');


  SELECT *
  FROM test;

  // NOT NULL == NULL
  // INSERT INTO test (id, name) VALUES (1, NULL); ERRO
  // INSERT INTO test (id, name) VALUES (NULL, NULL); ERRO

  
```

- FOREIGN KEY 

```

  // 3 Tables: names, course, student; 

  CREATE TABLE names (
    id INTERGER,
    name VARCHAR(255) NOT NULL
  );

  INSERT INTO names (id, name) VALUES (1, 'Maria');
  INSERT INTO names (id, name) VALUES (2, 'Uberto');

  SELECT *
  FROM names;


  CREATE TABLE course (
    id INTEGER,
    name VARCHAR(255) NOT NULL
  );
  
  INSERT INTO course (id, name) VALUES (1, 'HTML');
  INSERT INTO course (id, name) VALUES (2, 'CSS');

  SELECT *
  FROM course;


  CREATE TABLE student (
    id_name INTERGER,
    id_course INTERGER,
    PRIMARY KEY (id_name, id_course),

    FOREIGN KEY (id_name)
    REFERENCE names (id),

    FOREIGN KEY (id_course)  
    REFERENCE course (id)
  );
   
  INSERT INTO student (id_name, id_couser) VALUES (1,1);
  INSERT INTO student (id_name, id_couser) VALUES (1, 2);

  SELECT *
  FROM student;
  
```

- JOIN: to join tables with the same amount of data.

```

  SELECT *
  FROM names
  
  JOIN student ON student.id_name = names.id
  JOIN course  ON course.id       = student.course.id

  INSERT INTO student (id_name, id_course) VALUES (2,2); 

  
  SELECT names.name  as "Student Names",
         course.name as "Student Course"
  
  FROM names
  
  JOIN student ON student.id_name = names.id
  JOIN course  ON course.id       = student.course.id

  INSERT INTO student (id_name, id_course) VALUES (2,2); 
    
```

- LEFT: returns all rows from the left table, and the matched rows from the right table

```
  INSERT INTO student 
  
  SELECT * 
  FROM names

  LEFT JOIN student ON student.id_name = names.id
  LEFT JOIN course ON course.id        = student.course.id

```

- RIGHT: returns all rows from the right table, and the matched rows from the left table"

```
  INSERT INTO student 
  
  SELECT * 
  FROM names

  RIGHT JOIN student ON student.id_name = names.id
  RIGHT JOIN course ON course.id        = student.course.id

```

- FULL: return all 

``` 
  INSERT INTO student 
  
  SELECT * 
  FROM names

  FULL JOIN student ON student.id_name = names.id
  FULL JOIN course ON course.id        = student.course.id

```

- CROSS

```
  
  INSERT INTO student 
  
  SELECT * 
  FROM names

  CROSS JOIN student

```

- DELETE (Restrict or Cascade)

  - Restrict: standard, you cannot delete an element from a table associated with another table
  - Cascade: allowed to delete elements present in more than one table
  
```

  // 3 Tables: names, course, student; 

  CREATE TABLE names (
    id INTERGER,
    name VARCHAR(255) NOT NULL
  );

  INSERT INTO names (id, name) VALUES (1, 'Maria');
  INSERT INTO names (id, name) VALUES (2, 'Uberto');

  SELECT *
  FROM names;


  CREATE TABLE course (
    id INTEGER,
    name VARCHAR(255) NOT NULL
  );
  
  INSERT INTO course (id, name) VALUES (1, 'HTML');
  INSERT INTO course (id, name) VALUES (2, 'CSS');

  SELECT *
  FROM course;


  CREATE TABLE student (
    id_name INTERGER,
    id_course INTERGER,
    PRIMARY KEY (id_name, id_course),

    FOREIGN KEY (id_name)
    REFERENCE names (id)
    ON DELETE CASCADE,

    // standard 
    //ON DELETE RESTRICT
    
    FOREIGN KEY (id_course)  
    REFERENCE course (id)
  );
   
  INSERT INTO student (id_name, id_couser) VALUES (1,1);
  INSERT INTO student (id_name, id_couser) VALUES (1, 2);

  SELECT *
  FROM student;
 
  DELETE FROM name WHERE id = 1;

```

- UPDATE 

```

// 3 Tables: names, course, student; 

  CREATE TABLE names (
    id INTERGER,
    name VARCHAR(255) NOT NULL
  );

  INSERT INTO names (id, name) VALUES (1, 'Maria');
  INSERT INTO names (id, name) VALUES (2, 'Uberto');

  SELECT *
  FROM names;


  CREATE TABLE course (
    id INTEGER,
    name VARCHAR(255) NOT NULL
  );
  
  INSERT INTO course (id, name) VALUES (1, 'HTML');
  INSERT INTO course (id, name) VALUES (2, 'CSS');

  SELECT *
  FROM course;


  CREATE TABLE student (
    id_name INTERGER,
    id_course INTERGER,
    PRIMARY KEY (id_name, id_course),

    FOREIGN KEY (id_name)
    REFERENCE names (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
 
    
    FOREIGN KEY (id_course)  
    REFERENCE course (id)
  );
   
  INSERT INTO student (id_name, id_couser) VALUES (1,1);
  INSERT INTO student (id_name, id_couser) VALUES (1, 2);

  SELECT *
  FROM student;
 
  UPDATE name
  SET id = 10
  WHERE id = 2;

```

- Data query - Create basic table

```
  CREATE TABLE employees {
    id       SERIAL PRIMARY KEY,
    register VARCHAR(10),
    name     VARCHAR(255),
    last     VARCHAR(255) 
  };

  INSERT INTO employees (register, name, last) VALEUS ('M001', 'Roberto', 'Peron');
  INSERT INTO employees (register, name, last) VALEUS ('M002', 'Ana', 'Silva');
  INSERT INTO employees (register, name, last) VALEUS ('M003', 'Julia', 'Oliveira');
  INSERT INTO employees (register, name, last) VALEUS ('M004', 'Pedro', 'Almeida');
  INSERT INTO employees (register, name, last) VALEUS ('M005', 'Carlos', 'Santos');
  INSERT INTO employees (register, name, last) VALEUS ('M006', 'Antonieta', 'Ada');

  or

  INSERT INTO employees (register, name, last) VALUES 
  ('M002', 'Ana', 'Silva'),
  ('M003', 'Carlos', 'Santos'),
  ('M004', 'Julia', 'Oliveira'),
  ('M005', 'Pedro', 'Almeida'),
  ('M006', 'Maria', 'Fernandes');
 
```

- ORDER (ASC and DESC)

```
  SELECT * 
  FROM employees
  ORDER BY name DESC;

  SELECT *
  FROM employees
  ORDER BY register, name DESC;

  SELECT *
  FROM employees
  ORDER BY 3,4,2;

  SELECT *
  FROM employees
  ORDER last DESC, name DESC, register ASC;
  
```

- LIMIT, OFFSET

```
 SELECT *
 FROM employees
 LIMIT 5;

 SELECT *
 FROM employees
 ORDER BY name
 LIMIT 3;

 SELECT *
 FROM employees
 ORDER BY name
 LIMIT 3
 OFFSET 0;

 SELECT *
 FROM employees
 ORDER BY name
 LIMIT 4
 OFFSET 1;
  
```

- COUNT, SUM, MAX, MIN, AVG

```
 SELECT COUNT (id)
 FROM employees;

 SELECT COUNT (*)
 FROM employees;


 SELECT COUNT (id),
        SUM   (id)
 FROM employees;

 SELECT COUNT (*),
        SUM   (id)
 FROM employees;


 SELECT COUNT  (id),
        MAX    (id),
        SUM    (id)
 FROM employees;


 SELECT COUNT  (id),
        MAX    (id),
        MIN    (id),
        SUM    (*)
 FROM employees;


 SELECT COUNT (id),
        MAX   (id),
        MIN   (id),
        AVG   (id),
        SUM   (id)
 FROM employees;



 SELECT COUNT (id),
        MAX   (id),
        MIN   (id),
        ROUND (AVG(id),0),
        SUM   (id)
 FROM employees;

 SELECT COUNT (id),
        MAX   (id),
        MIN   (id),
        ROUND (AVG(id),2),
        SUM   (id)
 FROM employees;
  
```

- Date repeat ()

```
 SELECT DISTINCT name 
 FROM employees
 ORDER BY name;


 SELECT name, last, 
        COUNT (id)
 FROM employees
 GROUP BY name, last
 ORDER BY name;  
  
```
