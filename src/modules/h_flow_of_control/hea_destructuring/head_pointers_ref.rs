#![allow(dead_code, unused_imports)]

// for pointers, a distinction needs to be made between destructuring and
// dereferencing as they are different concepts which are used differently
// from languages like C/C++
// - dereferencing use *
// - destructuring uses &, ref, and ref mut
pub fn head_pointers_ref() {
    // assign a reference of type `i32` . the `&` signifies there
    // is a reference being assigned
    let reference = &4;

    match reference {
        // if `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32` 
        // `&val`
        // ^ we see that if the matching `&`s are dropped, then the `i32` should be 
        // assigned to `val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // to avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val)
    }

    // what if you don't start with a reference? `reference` was a `&` because
    // the right side was already a reference. This is not a reference because the right
    // side is not one.
    let _not_a_reference = 3;

    // rust provides `ref` for exactly this purpose. it modifies the assignment
    // so that a reference is created for the element; this reference is assigned
    let ref _is_a_reference = 3; // it is the same with `let _is_a_reference = &3;`

    // accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`
    let value = 5;
    let mut mut_value = 6;

    // use `ref` keyword to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // got a reference (&mut i32). gotta dereference it before
            // we can add anyting to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m) 
        }
    }
}