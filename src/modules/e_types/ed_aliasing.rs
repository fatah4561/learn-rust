#![allow(dead_code, unused_imports)]
// the type statement can be used to give a new name to an existing type.
// Types must have UpperCamelCase names, or the compiler will raise a warning.
// The exception to this rule are the primitive types: usize, f32, etc

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

pub fn ed_aliasing() {
    // `NanoSecond` = `Inch` = `U64` = `u64`
    let nanosecond: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    // Note that type aliases *don't* provide any extra type safety,
    // because aliases are *not* new types
    println!(
        "{} nano seconds + {} inches = {} unit?, {:?}",
        nanosecond,
        inches,
        nanosecond + inches,
        nanosecond + inches,
    )

    // the main use of aliases is to reduce boilerplate; for ex:
    // the `io::Result<T>` type is an alias for the `Result<T, io::Error>` type
}
