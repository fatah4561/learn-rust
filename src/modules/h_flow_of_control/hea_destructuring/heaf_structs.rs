#![allow(dead_code, unused_imports)]

// similarly, a struct can be destructured
pub fn heaf_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // try changing the values in the struct to see what happens
    let foo = Foo {x: (1,2), y: 3}; // 1st
    let foo = Foo {x: (2,2), y: 2}; // 2nd
    let foo = Foo {x: (2,2), y: 3}; // 3rd

    match foo {
        Foo {x: (1,b), y} => println!("First of x is 1, b = {}, y = {}", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo {y: 2, x: i} => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo {y, ..} => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field x
        // Foo {y} => println!("y = {}", y)
    }

    let faa = Foo {x: (1, 2), y: 3};

    // you do not need a match block to destructure structs:
    // the new variables is on the right side ex x: x0 (mean x0 is a new variable
    // from destructuring)
    let Foo {x: x0, y: y0} = faa;
    println!("outside: x0 = {x0:?}, y0 = {y0}");

    // destructuring works with nested structs as well:
    struct Bar {
        foo: Foo,
    }

    let bar = Bar {foo: faa};
    // let Bar {foo: foo} = bar;
    let Bar {foo: Foo {x: nested_x, y: nested_y}} = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}