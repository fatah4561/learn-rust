#![allow(dead_code, unused_imports)]
// the from and into traits are inherently linked, and this is actually part of its
// implementation. if you are able to convert type A from type B, then it should
// be easy to believe that we should be able to convert type B to type A.

use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

// implement only one either into or from
// because
// Specifically, Rust's standard library already provides a generic implementation 
// of Into for any type that implements From

// from
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}

// into
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number {value: self}
//     }
// }

// Notes to self to convert it back to i32
// we need to implement Number to i32
// ex:
// impl From<Number> for i32 {
//     fn from(number: Number) -> Self {
//         number.value
//     }
// }

pub fn fa_from_and_into() {
    // -- From
    // the from trait allow for a type to define how to create itself from another type
    // hence providing a very simple mechanism for converting between several types
    // there are numerous implementations of this trait within the standard library
    // for conversion of primitive and common types

    // for example we can easily convert a str into a String
    let my_str = "hello";
    let my_string = String::from(my_str);

    // we can do similar for defining a conversion of our own type.
    // import this `use std::convert::From;`
    // let num = Number::from(30);
    // println!("my number is {:?}", num)

    // ---- Into
    // the into trait is simply the reciprocal of the From trait.
    // it defines how to convert a type into another type
    // calling into() typically requires us to specify the result type
    // as the compiler is unable to determine this most of the time.
    let int = 5;
    // try removing the type annotation (got compile time error)
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // let num = Number::from(32);

    // -- From and Into are interchangeable
    // from and into are designed to be complementary.
    // we do not need to provide implementation for both traits.
    // if you have implemented the From trait for your type,
    // Into will call it when necessary. Note however that the converse
    // is not true: implementing Into for your type will not automatically
    // provide it with an implementation of `From`
    let num = Number::from(30);
    println!("my number is {:?}", num);

    let num: Number = int.into();
    println!("My number is {:?}", num);
}