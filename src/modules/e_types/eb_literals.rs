#![allow(dead_code, unused_imports)]
// numeric literals can be type annotated by adding the type as suffix.
// example to specify that literal 42 should have the type i32, then write 
// `42i32`
// the type of unsuffixed numeric literals will depend on how they are used.
// if no constraints exists the compiler will use i32 for integers, and f64 for
// floating point numbers
pub fn eb_literals() {
    // suffixed literals, their types are known at initialization
    // let x = 1u8;
    let x = 255u8;
    let y = 2u32;
    let z = 3f32;

    // unsuffixed literals, their types depend on how the are used
    let i = 1;
    let f = 1.0;
    
    // `size_of_value` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // `std::mem::size_of_val` is a function but called with its full path
    // code can be split in logical units called modules. in this case
    // the `size_of_val` function is defined in `mem` module, and the `mem`
    // is defined in the `std` create
}