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

- 2 item 4
