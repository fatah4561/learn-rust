#![allow(dead_code, unused_imports)]
// branching if - else is similar to other language. Unlike many of them
// the boolean condition doesn't need to be surrounded by parentheses
// and each condition is followed by a block, if - else conditionals are
// expressions, and, all branches must return the same type

pub fn ha_if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
        // return 6
        // ^ uncomment this to get compile time error
    }

    let big_n = if n < 10 && n > -10 {
        println!(", and is a big number, increase ten-fold");

        // this expression returns an `i32`
        10 * n
    } else {
        println!(", and is a big number, halve the number");

        // this expression must return an `i32` as well
        n / 2
        // n / 2;
        // ^ adding semicolon above will get return () and get
        // compile time error because the return type is differ
    }; // put semicolon here, all let bindings need it
    println!("{} -> {}", n, big_n)
}
