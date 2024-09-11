# PostgreSQL: Views, Sub-Consultas e Funções

-  Create database

```
  CREATE DATABASE study;

  CREATE TABLE name (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    date_birth DATE NOT NULL
  );


  // DELETE: DROP TABLE student, category;


  CREATE TABLE category (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL
  );


  CREATE TABLE course (
    id SERIAL PRIMARY KEY,
    course_name VARCHAR(255) NOT NULL,
    id_category INTEGER NOT NULL REFERENCES category(id)
  );

  
  // foreign key  
  CREATE TABLE student (
    id_name INTEGER NOT NULL REFERENCES name(id),
    id_course INTEGER NOT NULL REFERENCES course(id),
    PRIMARY KEY (id_name, id_course)
  );

  
```
- Insert data

```
  INSERT INTO name (first_name, last_name, date_birth) 
  VALUES ('Marta', 'Dias', '1980-05-10'), 
         ('Luiz', 'Roberto', '1990-01-02'),
         ('Julia', 'Leite', '1992-07-07'),
         ('Carlos', 'Marinho', '1994-11-30'); 

  INSERT INTO category (category_name) VALUES ('Front-end', 'Back-end', 'Data Science', 'Database');

  INSERT INTO course (course_name, id_category) 
  VALUES ('HTML', 1),
         ('CSS', 1),
         ('Js', 1),
         ('Java', 2), 
  
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
- 

```
  
```
