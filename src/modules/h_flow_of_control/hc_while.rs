#![allow(dead_code, unused_imports)]

pub fn hc_while() {
    // the `while` keyword can be used to run a loop while a condition is true.
    // write FizzBuzz using a while loop

    // counter variable
    let mut n = 1;

    // loop while `n` is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        }

        // increment counter
        n += 1;
    }
}