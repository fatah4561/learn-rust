#![allow(dead_code, unused_imports)]

// rust provide pattern matching via the `match` keyword, which can be used
// like a C switch the first matching arm is evaluated and all possible values
// must be covered

pub fn he_match() {
    let number =13;
    // ^try different values for number

    println!("Tell me about {}", number);
    match number {
        // match a single value
        1 => println!("One!"),
        // match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // ^try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("a teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // ^ try commenting this catch-all arm
        // (got compile time error `ensure all possible case covered...`)
    }

    let boolean = true;
    // match is an expression too
    let binary = match boolean {
        // the arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // ^try commenting out one of these arms
        // (got compile time error missing match arm)
    };

    println!("{} -> {}", boolean, binary);
}