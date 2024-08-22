#![allow(dead_code, unused_imports)]

fn age() -> u32 {
    // 15
    23
}

fn some_number() -> Option<u32> {
    Some(42)
}

pub fn hec_binding() {
    // indirectly accessing a variable makes it impossible to branch and use that 
    // variable without re-binding. `match` provides the @ sigil for binding values
    // to names

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // could `match 1..=12` directly but then what age
        // would the child be? instead, bind to `n` for the sequence of 1 .. =12
        // now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..=19 => println!("I'm a teen of age {:?}", n),
        // nothing bound return the result
        n => println!("I'm an old person of age {:?}", n)
    }

    // you can also use binding to `destructuring` enum variants, such as option:
    match some_number() {
        // got `Some` variant, match if its value, bound to `n`,
        // is equal to 42
        Some(n @ 42) => println!("The answer: {}", n),
        // match any other number.
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
