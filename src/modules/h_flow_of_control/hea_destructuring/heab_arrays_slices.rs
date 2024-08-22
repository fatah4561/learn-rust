#![allow(dead_code, unused_imports)]

// arrays and slices can be destructured like tuples
pub fn heab_arrays_slices() {
    // try changing the values in the array or make it slice
    let array = [1, -2, 6]; // 2nd
    let array = [0, -2, 6]; // 1st
    let array = [-1, -2, 6]; // 3rd
    let array = [3, -2, 6]; // 4th
    let array = [4, -2, 6]; // 5th
    // let array = [4, -2, 6, 8];
    let array = &array; // slice
    let array = &array[0..3]; // this slice goes to 5th arm
    // let array = &array[0..4]; // this give error range index 4 out of range

    match array {
        // binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] => {}", second, third),

        // single values can be ignored with _
        [1, _, third] => println!("array[0] = 1, array[2] = {} and array [1] was ignored", third),

        // you can also bind some and ignore the rest
        [-1, second, ..] => println!("array[0] = -1, array[1] = {}, and all the other ones were ignored", second),

        // this code would not compile
        // [-1, second] => ...,

        // or store them in another array/slice (the type depends on
        // that of the value that is being matched againts)
        [3, second, tail @ ..] => println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),

        // combining these patterns, we can, for example, bind the first and last values
        // and store the rest of them in a single array
        [first, middle @ .., last] => println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),

        _ => println!("no match found")
    }

    let four_size_array = [1,2,3,4];
    match four_size_array {
        [first, middle @ .., last] => println!("array[0] is {}, middle is {:?}, and last element is {}", first, middle, last),
        _ => println!("no match found")
    }

    // hmm so in summary the matching patterns we must knows the array data
    // and match the pattern by each element one by one using their index [1,2,3] match to
    // [0, 1, 2]
}