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

  ```
    // Variaveis
    // let nome = "valor";
  
    let mut name = "Alice";
    println!("Qual o seu nome: {}", name);
    println!("------------------------------");
      
    name = "Carlos";
    println!("Qual o seu nome: {}", name);
    println!("------------------------------");
      

    let mut num = 24;
    println!("Qual número você escolheu? {}", num);
    println!("------------------------------");
      
    num = 12;
    println!("Qual número você escolheu? {}", num);
    println!("------------------------------");
      

    let mut letter = 'a';
    println!("Qual é a primeira letra que vem à sua mente? {}", letter);
    println!("------------------------------");
      
    letter = 'b';
    println!("Qual é a primeira letra que vem à sua mente? {}", letter);
    println!("------------------------------");
      

    let mut ok = true;
    println!("1 + 1 é igual a dois? {}", ok);
    println!("------------------------------");

    ok = false;
    println!("1 + 1 é igual a dois? {}", ok);
    println!("------------------------------");

  ```

  Examples: 
  [Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1f539fc55724489f8dda00e02fe2c14f)


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

  Examples:
  [Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=df8f098992b4bf3a61ee22f1663dbcb4)

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

  Examples:
  [Acess](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=023a467eb4c4a7b4dbce9d12eeb05620)
