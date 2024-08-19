// variable bindings are immutable by default
// use mut modifier to set as mutable
#![allow(dead_code, unused_imports)]
pub fn db_mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok 
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // error can't assigns new value to immutable variable
    // _immutable_binding += 1;
}