# Rust

3. Functions and Methods

a. Functions;

- Functions: reusable code blocks that perform specific tasks;

```

  // 
  fn main () {}
  
```

- fn = function
- main = name
- () = specifications
- {} = code


- Example:

```

  fn main () {
    study();
  }
  
  fn study () {}
  
```

- Function (action) - the function that performs a specific action in your code. 

```

fn main () {
  print();
}

fn prin () {
  println!("Hellow, world!");
}

// Output
Hellow, world!
  
```

- Functions with parameters

```

fn main () {

  // Calling the function directly and passing the arguments 
  println!("{:?}", name("Mashu", "kyrielight"));
  
  // Assigning the function's return value to a variable
  let result: String = name("Mashu", "kyrielight");
  println!("{:?}", result);

}

fn name(first: &str, last: &str) -> String {
  let full_name = format!("{0} {1}", first, last);
  return full_name;
}
  
```

- fn = funcion
- name = function name
- () = specifictation
- first = first parameter name
- &str = specifies the type of the parameter
- last = second parameter name
- &str = specifies the type of the parameter
- -> = function return type
- String = specifies that the function will return a string


- Function (dead code): used to issue a warning about an unused function.

```

// Message
#[warn(dead_code)]
println!("Dead code!");

// Not message
#[allow(dead_code]
println!("Dead code!");
 
```

- Example:

```

fn main () {

  //prin();
  
}

#[warn(dead_code)]
println!("Dead code!");

// Output
warning: function `prin` is never used
  --> src/main.rs:10:4
   |
4  | fn prin() {
   |    ^^^^
   |
  
```

- Example: 

```
fn main () {

  //prin();
  
}

#[allow(dead_code]
println!("Dead code!");

// Output
not message

```

b. Modules;

- Modules: allows you to organize your code into different files that can bu used in the main function.

```

// create file my_module.rs
pub fn my_function() {
    println!("Olá do módulo my_module!");
}

// enter main.rs add
mod my_module;

fn main() {
    my_module::my_function();
}
  
```

- pub (public): makes an item visible outside the module;
- No pub (public): The item is only visible within the module.


- Module extern

```

  // cargo new hello! =)
  hello/
  ├── src/
  │   ├── main.rs
  │   └── test/
  │       ├── mod.rs
  │       └── my.rs         


  // main.rs
  mod test;

  fn main() {
      println!("Hello, world!");

      test::my::fun();
      // or crate::ssemantic::my::fun();
  }

  // mod.rs
  pub mod my;


  // my.rs
  pub fn fun(){
    println!("congratulations!");
    
  }
  
  
```
