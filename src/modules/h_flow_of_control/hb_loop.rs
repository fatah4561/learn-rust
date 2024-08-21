#![allow(dead_code, unused_imports)]

// rust provides a `loop` keyword to indicate an infinite loop
// the `break` statement can be used to exit a loop at anytime
// whereas the `continue` statement can be used to skip the rest of 
// the iteration and start a new one

pub fn hb_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // skip the rest of this iteration (u will se that 3 is not printed)
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("Ok, that's enough!");

            // exit this loop
            break;
        }
    }
}