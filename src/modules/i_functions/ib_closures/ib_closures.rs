#![allow(dead_code, unused_imports)]

/*
    closures are functions that cane capture the enclosing environment
    for example closures that captures the x variable:
    |val| val + x
    the syntax and capabilities of closures make them very convenient for
    on the fly usage. Calling a closure is exactly like calling a funcion.
    however both input and return types can be inferred and input variable
    names must be specified

    other characteristics of closures include:
    - using || instead of () around input variable
    - optional body delimitation ({}) for a single line expression (mandatory
    otherwise)
    - the ability to capture the outer environment variables

    p.s summary syntax sugar yep another way to define and use function
*/
pub fn ib_closures() {
    let outer_var =42;

    // a regular functions can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 {i + outer_var}
    // ^ will give compile time error to suggest using closure instead

    let closure_annotated = |i: i32| -> i32 {i + outer_var};
    let closure_inferred = |i     |          i + outer_var  ;
    // ^ ps if u write this but not using it the compiler will get type error
    // but after adding the below closure use then it got that the type is i32
    // safer to use annotated instead

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure inferred: {}", closure_inferred(1));
    // once closure's type has been inferred, it cannot be inferred again
    // println!("closure_inferred: {}", closure_inferred(1_u32));
    // ^uncommenting it will give compile time error

    // a closure taking no arguments which returns an `i32`
    // the return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one());
    
;}