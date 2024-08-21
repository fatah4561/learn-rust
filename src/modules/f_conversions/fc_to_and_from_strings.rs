#![allow(dead_code, unused_imports)]

// converting to string
// to convert any type to a String is as simple as implementing the ToString
// trait for the type. Rather than doing so directly, you should implement the
// fmt::Display trait which automagically provides ToString and also allows
// printing the type as discussed in the section on print!

use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn fc_to_and_from_strings() {
    let circle = Circle{radius: 6};
    println!("{}", circle.to_string());

    // -- Parsing a String
    // it's useful to convert strings into many types, but one of the more
    // common string operations is to convert them from string to number.
    // The idiomatic approach to this is to use the `parse` function
    // and either to arrange for type inference or to specify the type to
    // parse using the 'turbofish' syntax. below

    // this will convert the string into the type specified as long as the FromStr
    // trait is implemented for that type. This is implemented for numerous types
    // within the standard library. TO obtain this functionality on a user
    // defined type simply implement the FromStr for that type

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}