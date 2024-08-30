#![allow(dead_code, unused_imports)]

/*
    closures are inherently flexible and will do what the functionality
    requires to make the closure work without annotation, this allows
    capturing to flexibility adapt to the use case, sometimes moving 
    and sometimes borrowing. closures can capture variables:
    - by reference: &T
    - by mutable reference: mut &T
    - by value: T

    ^they preferentially capture variables by reference and only go lower when
    required
*/
pub fn iba_capturing() {
    use std::mem;

    let color = String::from("green");

    // a closure to print color which immediately borrows (`&`) color
    // stores the borrow and closure in the print variable. it will remain
    // borrowed until print is used the last time.
    // println! only requires arguments by immutable reference so it doesn't
    // impose anything more restrictive
    let print = || println!("`color`: {}", color);

    // call the closure using the borrow
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`
    let _reborrow = &color;
    print();

    // a move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // a closure to increment `count` could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. immediately
    // borrows `count`
    // a `mut` is required on `inc` because a `&mut` is stored inside. Thus,
    // calling the closures mutates `count` which requires a `mut`
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // call the closure using a mutable borrow
    inc();
    // println!("count outside: {}", count); // also become one (well mutable)
    // ^only uncomment if `count` is no longer used below

    // the closure still borrows `count` because it is called later
    // an attempt to reborrow will lead to an error
    // let _reborrow = &count;
    // ^compile time error, bcs below we called the function again
    inc();

    // the closure no longer needs to borrow `&mut count`. therefore, it is
    // possible to reborrow without an error
    let count_reborrowed = &mut count;
    println!("count reborrowed: {}", count_reborrowed);

    // a non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires `T` so this must take by value, a copy type
    // would copy into the closure leaving the original untouched.
    // a non-copy type must move and so `movable` immediately moves into
    // the closure
    let consume = || { 
        println!("`movable`: {:?}", movable);
        mem::drop(movable); // dang it is just actually moving it inside function
        // so it is gone from here -.-
    };
    // `consume` consumes the variable so this can only be called once
    consume();
    // consume();
    // ^compile time error

    // using `move` before vertical pipes forces the closure to take ownership
    // of captured variables:
    // `Vec` has non-copy semantics
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    // let contains = |needle| haystack.contains(needle);
    // ^without move

    println!("{}", contains(&1));
    println!("{}", contains(&2));

    // println!("There're {} elements in vec", haystack.len()); // won't compile
    // ^compile time error
    // because borrow checker doesn't allow re-using variable after it has been moved

    // removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting the above line will works


    // println!("There're {} elements in vec", contains.len()); // won't compile

}