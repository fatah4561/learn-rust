#![allow(dead_code, unused_imports, overflowing_literals)]
// rust have no implicit type conversion (coercion)
// but rust has explicit type conversion (casting)

// rust for converting between integral type follow C conventions generally
// except in case where C has undefined behavior
pub fn ea_casting() {
    // let decimal = 65.4321_f32;
    let decimal = 65.6321_f32;

    // error no implicit conversion
    // let integer: u8 = decimal;
    // ^uncomment to show compiler time error

    // explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // error: there are limitation in conversion rules
    // a float can't be directly converted to a char
    // let character = decimal as char;
    // ^uncomment to show compiler time error

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::Max + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated
    // (the 1000 number in binary is 1111101000 req more than 8 bits)
    // (so if this number is stored in 8 bit it will be truncated only the first)
    // (8 bits from the right are kept (LSB)  1111101000 to 8 bits gives 11101000)
    println!("1000 as u8 is: {}", 1000 as u8);
    // ^hmm got compile time error use the #![allow(overflowing_literals)]
    // -1 + 256 = 255 (since there is no sign bit in u8)
    println!(" -1 as u8 is: {}", (-1i8) as u8); 

    // for positive numbers, this is the same as modulus
    println!("1000 mod 256 is {}", 1000 % 256);

    // when casting to a signed type, the (bitwise) result is the same
    // as first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative
    println!("130u8 as i8 is: {}", 130u8 as i8);
    println!("130u8 as i16 is: {}", 130u8 as i16); // it fits so the value stay
    println!("-130i8 as u8 is: {}", -130i8 as i8);
    println!("-130i8 as u16 is: {}", -130i8 as i16);

    // unless it already fits, of course
    println!("128 as a i16 is: {}", 128 as i16);

    // in boundary case 128 value is in 8-bit two's complement
    // representation is -128 (dk still not quite get it yet)
    println!("128 as i8 is: {}", 128 as i8);
    println!("128 as i8 is: {}", 129 as i8);
    println!("128 in binary is: {:08b}", 128);
    println!("128 i8 in binary is: {:08b}", 128 as i8);
    println!("128 u8 in binary is: {:08b}", 128 as u8);
    // bcs the range of i8 is -128 to 127

    println!("==========");

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is {}", 1000 as u8); // literal overflows
    // and the value of the 232 in 8-bit two's complement representation is -24
    println!("232 as a i8 is : {}", 232 as i8); // literal overflows

    // since rust 1.45 the `as` keyword performs a *saturated cast*
    // (casting to nearest valid value == saturated cast) when casting from float to int
    // if the floating value exceeds the upper bound or is less than the lower bound,
    // the returned value will be equal to the bound crossed

    // 300.0 as u8 is 255
    println!("300.0 as u8 is {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("nan as u8 is {}", f32::NAN as u8);

    // this behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values** use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!("300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}