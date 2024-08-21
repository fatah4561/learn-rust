#![allow(dead_code, unused_imports, unused_must_use, path_statements)] // -.- well learning purpose so

// A rust program is (mostly) made up of a series of statements
pub fn g_expressions() {
    // statement
    // statement
    // statement

    // there are a few kinds of statement in Rust. the most common two are
    // declaring variable binding, and using a `;` with an expression:
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // blocks are expressions too, so they can be used as value in assignments
    // the last expression in the block will be assigned to the place expression
    // such as a local variable, however if the last expression of the block ends
    // with a semicolon, the return value will be ()
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // this expression will be assigned to `y` (got u32 in y)
        x_cube + x_squared + x
    };

    let z = {
        // the semicolon suppresses this expression and `()` is assigned
        // to `z` (got () in z)
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}