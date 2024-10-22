//use std::io;

pub fn hackerrank() {
    println!("-----------------------------------------");

    // Array
    let alice: [i16; 3] = [5, 5, 6];
    let bob: [i16; 3] = [4, 5, 6];

    // print - Alice
    for i in alice {
        println!("{}", i);
    }
    println!("-----------------------------------------");

    // Print - Bob
    for i in bob {
        println!("{}", i);
    }
    println!("-----------------------------------------");

    // Sum - Alice
    let mut salice = 0;
    for i in alice {
        salice += i;
    }
    println!("{}", salice);
    println!("-----------------------------------------");

    // sum - Bob
    let mut sbob = 0;
    for i in bob {
        sbob += i;
    }
    println!("{}", sbob);
    println!("-----------------------------------------");

    // Size Array: Alice and Bob
    if alice.len() != bob.len() {
        println!("{}", false);
    } else {
        println!("{}", true);
    }
    println!("-----------------------------------------");

    // Alice
    println!("Values of Alice: ");
    for i in 0..alice.len() {
        println!("{}", alice[i]);
    }

    // Bob
    println!("Values of Bob: ");
    for i in 0..bob.len() {
        println!("{}", bob[i]);
    }

    //
    let mut palice = 0;
    let mut pbob = 0;

    for i in 0..alice.len() {
        if alice[i] > bob[i] {
            palice += 1;
        } else if alice[i] < bob[i] {
            pbob += 1;
        }
    }

    println!("Alice: {}", palice);
    println!("Bob: {}", pbob);

    println!("-----------------------------------------");
}
