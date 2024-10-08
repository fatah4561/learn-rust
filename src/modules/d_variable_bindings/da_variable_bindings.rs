// variable bindings can be type annotated
// ex let an_integer = 1u32;
#![allow(dead_code, unused_imports)]
pub fn da_variable_bindings() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // the compiler warns about unused variable bindings; these warnings can 
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    // let noisy_unused_variable = 2u32;
    // ^uncomment to check compiler warning
    let _noisy_unused_variable = 2u32;
    // ^fixed
}