pub fn a2() {
    // Variables

    let name = "Alice";
    println!("What is your name: {}", name);
    println!("------------------------------");

    let num = 24;
    println!("What number did you choose? {}", num);
    println!("------------------------------");

    let letter = 'a';
    println!(
        "Whats is the firts letter that comes to your mind? {}",
        letter
    );
    println!("------------------------------");

    let ok = true;
    println!("One plus one equals two? {}", ok);
    println!("------------------------------");

    // Variables and Mutability

    // ERROR!
    //let firstname = "Alice";
    //firstname = "Jonas";
    //println!("{}", firstname);
    //println!("------------------------------");

    // CORRECT
    let mut firstname = "Alice"; // Add = mut
    firstname = "Jonas";
    println!("{}", firstname);
    println!("------------------------------");

    // Constant
    const TWO: i8 = 2;
    println!("{}", TWO);
    println!("------------------------------");

    // Shadowing
    let food = "bread";
    println!("{}", food);

    let food = "milk";
    println!("{}", food);

    let food = "pizza";
    println!("{}", food);
    println!("------------------------------");

    // Scope
    let pet = "cat";
    println!("{}", pet);
    {
        let pet = "dog";
        println!("{}", pet);
    }
    println!("{}", pet);
    println!("------------------------------");

    // scope
    let x = 1;
    println!("{}", x);

    {
        let x = 4;
        println!("{}", x);
    }

    println!("{}", x);
}
