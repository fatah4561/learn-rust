#![allow(dead_code, unused_imports)]
// it is possible to declare variable bindings first, and then initialized them later
// however this form is seldom used, as it may lead to the use of uninitialized variables.
pub fn dd_declare_first() {
    // declare a variable binding
    let a_binding;

    {
        let x = 2;

        // initialized the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // error use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // ^uncomment to check compiler error

    another_binding = 1;
    println!("another binding: {}", another_binding)
}