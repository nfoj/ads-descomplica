# Rust!

2. Syntax and Semantics

d. input

  - std

  ```

    // use std::io - data input library
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("Failed to read line!");


    // 
    let mut input01 = String::new();
    io::stdin().read_line(&mut input01).expect("Failed to read line");


    // Resume
    println!("Enter your name:");
    
    let mut name = String::new();
    io::stdin()
      .read_line(&mut name) 
      //.trim()
      .expect("Failed in read line!");

    println!("Hello, {}!", name.trim());
        
  ```
