#![allow(dead_code, unused_imports)]
// stable since: rust 1.65
// to target specific edition can be done by compiling like this
// `rustc --edition=2021 main.rs`

use core::panic;
use std::str::FromStr;

// with let-else, a refutable pattern can match and bind the variables in the
// surrounding scope like a normal let, or else diverge (eg. break or return or panic)
// when the pattern doesn't match

fn get_count_item(s: &str) -> (u64, &str) { 
    let mut it = s.split_whitespace();
    // let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
    //     panic!("Can't segment count item pair: '{s}'");
    // };

    // the scope of the name bindings is the main thing that makes this different
    // from match or if let - else expressions, you could previously approximate
    // these patterns with an unfortunate bit of repetition and an outer let
    
    // match version
    let (count_str, item) = match (it.next(), it.next()) {
        (Some(count_str), Some(item)) => (count_str, item),
        _ => panic!("Can't segment count item pair: '{s}'"),
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse count: '{count_str}'");
    };

    // if let version
    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("Can't parse count: '{count_str}'");
    };
    (count, item)
}

pub fn hg_let_else() {
    assert_eq!(get_count_item("61 pennies"), (61, "pennies"));
    // assert_eq!(get_count_item("61 pennies"), (6, "pennies"));
    // assert_eq!(get_count_item("6l pennies"), (6, "pennies"));
    // assert_eq!(get_count_item("six pennies"), (6, "pennies"));

    // summary this is variable bindings with matching patterns
}