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
  VALUES('HTML', 1),
        ('CSS', 1),
        ('Js', 1),
        ('Java', 2),
        ('C#', 2),
        ('Rust', 2), 
        ('PostgreSQL', 3),
        ('MySQL', 3), 
        ('Oracle', 3), 
        ('SQL Server', 3),
        ('SQLite', 3), 
        ('Pandas', 4), 
        ('Machine Learning', 4), 
        ('Power Bi', 4); 
  
  INSERT INTO student VALUES (1, 4),(1, 11), (2, 1), (2, 2), (3, 4), (3, 3), (4, 4), (4, 6), (4, 5);

  
```

- Basic

```
  SELECT * 
  FROM category
  WHERE id =4;


  // Change category name
  UPDATE category 
  SET name = 'Data-Science'
  WHERE id =4;


  SELECT * 
  FROM category;

  
```

- Print

```

  SELECT * 
  FROM name;


  SELECT * 
  FROM name
  JOIN student ON student.id_name = name.id
  JOIN course ON course.id = student.id_course;


  // Option 1
  SELECT name.first_name, 
         name.last_name,
         COUNT(course.id) list_courses
  FROM name
  JOIN student ON student.id_name = name.id
  JOIN course ON course.id = student.id_course
  
  //GROUP BY name.first_name, name.last_name; 
  //or
  GROUP BY 1,2
  ORDER list_courses DESC;


  // Option 2
  SELECT name.first_name,
         name.last_name,
         COUNT(course.id) AS list_courses
  FROM name
  JOIN stundent ON student.id_name = name.id
  GROUP BY 1,2
  ORDER list_courses DESC;
  
  // 1 Result
  LIMIT 1;
   
  
```

- Seach courses

```
  // All courses
  SELECT *
  FROM course
  LEFT JOIN student ON student.id_course = couse.id


  // Couser with student
  SELECT * 
  FROM course
  JOIN student ON student.id_course = course.id;


  // 
  SELECT couse.name,
         COUNT(student.id_name) numbers_student 
  FROM couse
  JOIN student ON student.id_course = course.id
  GROUP BY course.name; // or GROUP BY 1;
  ORDER BY numbers_student DESC;
  
```

- IN 

```
  SELECT *
  FROM course;
  
  SELECT *
  FROM category;


  SELECT *
  FROM course 
  WHERE id_category IN (1,2); // WHERE id_category = 1 OR id_category = 2;
  
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
