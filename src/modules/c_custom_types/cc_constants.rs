#![allow(dead_code, unused_imports)]
// constant can be declared in any scope including global
// const: unchangeable value (common case)
// static: a possibly mutable variable with `!static` lifetime,
// the static lifetime is inferred doesn't need to be specified,
//  accessing or modifying mutable static variable is `unsafe`

// Globals declared outside all other scopes
const THRESHOLD: i32 = 10;
static LANGUAGE: &str = "Rust";


fn is_big(n: i32) -> bool {
    // access constant in some func
    n > THRESHOLD
}

pub fn cc_constants() {
    let n = 16;

    // access constant in the main thread (maybe not but this func will be called
    // to main)
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) {"big"} else {"small"});

    // error can't modify a const
    // THRESHOLD = 10;
    // fixme ^ comment out this line
}