#![allow(dead_code, unused_imports)]

/*
    functions are declare using `fn` keyword its argument are type annotated
    just like variables, and, if the functions returns a value, the return
    type must be specified after an arrow `->`

    the final expression in the function will be used as return value.
    alternatively the `return` keyword can be used to return a value
    earlier from within the function, even from inside loops or `if`
    statements


    and unlike C/C++ there is no restriction on the order of function definition
*/

pub fn  i_functions() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // corner case early return
    if rhs == 0 {
        return false;
    }

    // this is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// functions that don't return a value actually return the unit type
// `()`

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// when a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}