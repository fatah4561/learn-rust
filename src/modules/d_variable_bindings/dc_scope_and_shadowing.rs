// variable bindings have a scope, and are constrained to live in a block
// a block is a collection of statements enclosed by braces {}
#![allow(dead_code, unused_imports)]
pub fn dc_scope_and_shadowing() {
    // this live in main function `dc_scope_and_shadowing`
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope than the main function
    {
        // this binding only exists in this block
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);
    } // end of block

    // error `short_lived_binding` doesn't exists in this scope
    // println!("outer short: {}", short_lived_binding);
    // ^uncomment to show compiler error

    println!("outer long: {}", long_lived_binding);

    // 

    /*
        variable shadowing is allowed
     */

    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);

        // this binding `shadows` the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);

    // this binding `shadows` the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}