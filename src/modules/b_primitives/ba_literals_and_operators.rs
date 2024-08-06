
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn a_literals_and_operators() {
    // 2.1 literals and operators
    let truest = true;
    let an_integer_32 = 50;
    let an_array = [1, 2, 3, 4, 5, 6];
    let a_tuple = (1, 2, true, false, "aaa");
    let single_char = 's';
    let mut multi_char = "oops";
    multi_char = "aw";

    // hexa assign int
    let hexa = 0xffff;

    // underscore in number make things easier
    let underscore_int = 1_000_000_000;

    // scientific e notation
    let science_nota = 1e6;

    // integer addition
    println!("1 + 2 = {}", 1i32 + 2);

    // integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // will overflow due to that 1 is lower than 2  ( 1 - 2 = -1 but the type is unsigned)
    // println!("1 - 2 = {}", 1u32 - 2);

    // scientific
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // short circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // bitwise operation
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("0011 << 2 is {:04b}", 0b0011u32 << 2);
    println!("0011 >> 2 is {:04b}", 0b0011u32 >> 2);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // underscore improve readability
    println!("One million can be written as {}", 1_000_000u32);
}
