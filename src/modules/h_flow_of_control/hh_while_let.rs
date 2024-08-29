#![allow(dead_code, unused_imports)]

pub fn hh_while_let() {
    // similar to if let, while let, can make awkward match sequences more tolerable
    // consider the following sequence that increments i

    // make optional of type Option<i32>
    let mut optional = Some(0);

    // repeatedly try this test
    loop {
        match optional {
            // if optional destructures evaluate the block.
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                } 
                // ^ require 3 indentations
            },
            // quit the loop when destructure fails:
            _ => { break; }
            // ^why should this be required there must be a better way
        }
    }

    // using while let makes this sequence much nicer
    let mut optional = Some(0);

    // this reads: "while `let` destructures `optional` into"
    // `Some(i)`, evaluate the block (`{}`). Else `break`
    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional=None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else` / `else if`
    // clauses. `while let` doesn't have these.
}