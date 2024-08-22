#![allow(dead_code, unused_imports)]

// a `match`guard can be added to filter the arm

enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

pub fn heb_guards() {
    let temperature = Temperature::Celsius(35);
    let temperature = Temperature::Celsius(29);
    // ^try different values for `temperature`

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // the `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 fahrenheit", t),
    }

    // note that the compiler won't take guard conditions into account when checking
    // if all patterns are covered by the match expression
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen"),
        // ^ uncomment to fix compilation
    }
}