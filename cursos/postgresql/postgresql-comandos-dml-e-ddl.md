# PostgreSQL: comandos DML e DDL

- Create table

```
  CREATE YABLE name (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    date_birth DATE NOT NULL
  );


  CREATE TABLE category (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE
  );


  CREATE TABLE course (
    id SERIAL PRIMARY KEY,
    course_name VARCHAR(255) NOT NULL,
    category_id INTERGER NOT NULL REFERENCES category(id)
  );


  CREATE TABLE name_course (
    name_id INTERGER NOT NULL REFERENCES name(id),
    course_id INTERGER NOT NULL REFERENCES course(id),
    PRIMARY KEY (name_id, course_id)
  );
  
```

- Schema: It's like a container that organizes and groups related database objects. These objects can include tables, views, functions, sequences, and others. 

```
  // Schema rename: finance = fin // academic = acad ...

  CREATE SCHEMA academic;

  // If you have a created table
  DROP TABLE name, category, course, name_course;

  CREATE TABLE academic.name (
    id SERIAL PRIMARY KEY, 
    first_name VARCHAR(255) NOT NULL, 
    last_name VARCHAR(255) NOT NULL,
    date_birth DATE NOT NULL
  );
  
  
  CREATE TABLE academic.category (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE
  );


  CREATE TABLE academic.course (
    id SERIAL PRIMARY KEY,
    course_name VARCHAR(255) NOT NULL,
    category_id INTERGER NOT NULL REFERENCES category(id)
  );


  CREATE TABLE academic.name_course (
    name_id INTERGER NOT NULL REFERENCES name(id),
    course_id INTERGER NOT NULL REFERENCES course(id),
    PRIMARY KEY (name_id, course_id)
  );
  
```

- Not exists

```

  // IF NOT EXISTS
  
  CREATE TABLE clients;
  CREATE TABLE IF NOT EXISTS clients; 

  // Notice: relation "clients" alredy exists ...

```

- Default

```

  // DEFAULT
  
  CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    date_birth DATE NOT NULL DEFAULT NOW()::DATE
  );


  CREATE TABLE employess (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL DEFAULT 'E1'
  );

```

- Check

```

  // CHECK // <> or !=

  CREATE TABLE students (
   id SERIAL PRIMARY KEY,
   first_name VARCHAR(255) NOT NULL,
   last_name VARCHAR(255) NOT NULL CHECK (last_name != '')
  );

```

- Temporary

```

  // TEMPORARY TABLE

  CREATE TEMPORARY TABLE name(
    column VARCHAR(255) NOT NULL CHECK(column != '')
  );

  INSERT INTO name VALUES ('');
  // ERRO

  INSERT INTO name VALUES ('Roberto');
  
```

- Remane

```
  CREATE TEMPORARY TABLE tb_a (
    column_a VARCHAR(255) NOT NULL
  );

  ALTER TABLE tb_a RENAME TO tb_b;
  
  SELECT * 
  FROM tb_b;


  ALTER TABLE tb_b RENAME column_a TO column_b; 
  
```

- Insert

```
  
  // '' = data  //  "" = text  //  '10.0' > 10.0 
  INSERT INTO table_a VALUES ('A', "A", 19.0);

  SELECT table_a 
  AS "Table A" 
  FROM tb_a;
  
```

- Import and Export

```
  Table import
    Format: .csv
    Encoding: UTF8
    Header: Yes or No
    Delimites: , . | [tab]
    Quote: ""
    Columns: select

  Table export
    Format: 
    Encoding: 
    ...
  
```
