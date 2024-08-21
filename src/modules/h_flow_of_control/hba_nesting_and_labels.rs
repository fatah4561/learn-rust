#![allow(dead_code, unused_imports)]

// it's possible to `break` or `continue` outer loops when dealing with nested loops
// in these cases, the loops must be annotated with some 'label, and the label
// must be passed to the break / continue statement

pub fn hba_nesting_and_labels() {
    'outer: loop {
        println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");

                // this would break only the inner loop
                // break;

                // this breaks the outer loop
                break 'outer
            }
        println!("this point will never be reached");
        // break
    }
    println!("Exited the outer loop");
}