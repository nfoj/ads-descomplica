# Rust

1. Introduction

2. Syntax and Semantics

    a. Variables, constants and data types;

        - print, comments and placeholders;
        - variables and mutability;
        - data types;

    b. Control flow constructs;
    
    c. Functions and method syntax;
    
    d. Pattern matching and destructuring.



    C. Functions and Modules


    - Functions

        Reusable code blocks that perform specific tasks, essential for organizing and structuring code, making it more readable, efficient, and maintainable.

        ```

            // main
            fn main () {

            let result = sum (1, 2);    
            println!("{}",result);
            
            
            }
                        

            fn sum (a: u8, b: u8) -> u8 {
                a + b
            }


            // No use function!
            #[allow(dead_code)]
            fn subtration (x: i8, y:i8) -> i8 {
                x - y
            }    
    
           
        ```

    
