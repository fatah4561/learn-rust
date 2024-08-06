use std::mem;

#[allow(dead_code, unused_imports)]

fn analyze_slice(slice: &[i32]) {
    println!("====start====");
    println!("First element of slice: {}", slice[0]);
    println!("Last element of slice: {}", slice[slice.len()-1]);
    println!("The slice has {} elements", slice.len());
    println!("====end====");
}
pub fn bc_arrays_and_slices() {
    // array: 
    // - same type, [], length known at compile time

    // slices:
    // length not known at compile time, `&[T]`, with pointer

    // fixed-size array (type signature is superfluous(unnecessary))
    let xs:[i32; 5] = [1,2,3,4,5];

    // all elements initialized with same value (make 0, 500 times)
    let ys: [i32;500] = [0; 500];

    // index start at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // len return count of the elements in array
    println!("Number of elements in array: {}", xs.len());

    // array are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // array can be automatically borrowed as slices.
    println!("Borrow the whole array as slice");
    analyze_slice(&xs);
    let slice : &[i32] = &xs;
    println!("Here is element 1 of the slice {}", slice[0]);

    // slices can point to a section of an array
    // [starting_index...ending_index]
    println!("Borrow some section of the array as slice");
    analyze_slice(&xs[3..4]);
    analyze_slice(&ys[1..4]);

    // empty slice `&[]`
    let empty_array: [u32; 0] = [];
    // on assertion if it assert true then no error returned
    // but if fail return error
    // assert_ne!(&empty_array, &[]); // example uncomment this
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same but more verbose

    // arrays can safely be accessed using `.get`
    // that return `Option`, 
    // or used with `.expect()` if the program want to be stopped with message
    // otherwise it will continue
    for i in 0..xs.len() + 1 { // 1 element too far
        match xs.get(i) {
            Some(xs_val) => println!("{}: {}", i , xs_val),
            None => println!("element not found on index {}", i),
        }
    }

    // indexing the usual way will cause error on out of bond index
    // on array it is compile time error
    // print!("{}", xs[5]); // cargo task failed

    // on slice it is runtime error
    // the `xs[..]` expression is used to create slice from array
    // println!("{}", xs[..][5]); // panicked index out of bound
}