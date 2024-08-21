#![allow(dead_code, unused_imports)]

pub fn hbb_returning_from_loops() {
    // one of the uses of a `loop` is to retry an operation until it succeeds
    // if the operation returns a value though, you might need to pass it 
    // to the rest of the code: by putting it after the `break` statement
    // and it will be returned by the `loop` expressions

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // why this one can use semicolon?
            // maybe because it is loop expression not if
        }
    };

    assert_eq!(result, 20);
    println!("{}", result);

}