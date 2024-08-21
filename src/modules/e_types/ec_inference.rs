#![allow(dead_code, unused_imports)]
// the type inference engine is pretty smart,
// it does more than looking at the type of the value expression
// during an initialization. it also looks at how the variable is used
// afterwards to infer its type.
pub fn ec_inference() {
    // because of type annotation the compiler knows that elem has type u8
    let elem = 5u8;

    // create an empty vector (a growable array) (same type).
    let mut vec = Vec::new();
    // at this point the compiler doesn't know the exact type of 
    // `vec` it just knows that it's a vector of something (`Vec<_>`)

    // insert `elem` in the vector
    vec.push(elem);
    // now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // ^ try to commenting out the `vec.push(elem)` line
    // (got compile time error type annotations is need for `Vec<_>`)

    // vec.push(String::from("ba"));

    println!("{:?}", vec);
}