#![allow(dead_code, unused_imports)]

pub fn heaa_tuples() {
    // tuples can be destructured in a match as follows:
    let triple = (0, -2, 3); // fist arm
    let triple = (1, -2, 3); // second arm
    let triple = (5, -2, 2); // third arm
    let triple = (3, -2, 4); // fourth arm
    let triple = (4, -2, 4); // fifth arm
    // ^try different values for `triple`

    println!("Tell me about {:?}", triple);
    // match can be used to destructure a tuple
    match triple {
        // destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is 1 and the rest doesn't matters"),
        (.., 2) => println!("Last is 2 and the rest doesn't matters"),
        (3, .., 4) => println!("First is 3 last is 4 and the rest doesn't matters"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    let player_status = (0, 5, (10, 20)); // 1st
    let player_status = (100, 5, (10, 20)); // 2nd
    let player_status = (2, 11, (10, 20)); // 3rd
    let player_status = (2, 11, (11, 20)); // 4th

    match player_status {
        (0, _, _) => println!("Player is dead"),
        (100, ammo, _) if ammo < 10 => println!("Player is at full health but low on ammo"),
        (_, _, (10, 20)) => println!("Player is in a critical position"),
        _ => println!("Player is fine"),
    }
}
