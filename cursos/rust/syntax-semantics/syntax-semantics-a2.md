# Rust!

2. Syntax and Semantics

b. variables and mutability;

- Variables

  - Variable are reserved memory spaces for storing information;

  ```
    let name = "Alice";
    println!("What is your name: {}", name);
    println!("------------------------------");

    let num = 24;
    println!("What number did you choose? {}", num);
    println!("------------------------------");

    let letter = 'a';
    pintln!("Whats is the firts letter that comes to your mind? {}", letter);
    println!("------------------------------");

    let ok = true;
    println!("One plus one equals two? {}", ok);
    println!("------------------------------");

  ```

  Examples: 
  [Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8433dc0e04c73205f692933d53514b12)


- Variables and Mutability

  - Add 'mut' after let to change the value of a variable.

    - let mut name = "Jonas";
    - let mut age = 40;
    - let mut city = "Mexico";
    - let mut pet = "Cat";


> [!WARNING]
> Erro!


  ```
    let name = "Alice";
    println!("What is your name: {}", name);

    // Erro
    name = "Jonas";
    pritnln!("What is your name: {}", name);

  ```

> [!WARNING]
> Correct!


  ```
    // Add: mut
    let mut name = "Alice";
    println!("What is your name: {}", name);

    // Correct
    name = "Jonas";
    pritnln!("What is your name: {}", name);

  ```

- Constant

  - A constant cannot be changed once it's been assigned a value;
  - Constants should be declared in uppercase and have an explicit type.

  ```
    // const
    const POINTS = 3;
    println!("{}", POINTS);


    // Erro!
    POINTS = 4;
    println!("{}", POINTS);

  ```

- Shadowing

  - Shadowing a variable by declaring a new one with the same name;

  ```
    // shadowing
    let food = "bread";
    println!("{}", food);

    let food = "milk";
    println!("{}", food);

    let food = "pizza";
    println!("{}", food);

  ```

- Scope

  - When a block of code is enclosed within curly braces {}, it enters a new scope, which is nested within the main program's scope.

  ```
    // scope
    let x = 1;
    println!("{}", x);

    {
      let x = 2;
      println!("{}", x);
    }

    println!("{}", x);

  ```
