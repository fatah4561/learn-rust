#![allow(dead_code, unused_imports)]

// for and range
pub fn for_and_range() {
    // the `for in` construct can be used to iterate through an `Iterator`
    // one of the easiest ways to create an iterator is to use the range 
    // notation a..b This yields values from a (inclusive) to b (exclusive)
    // in steps of one

    // write FizzBuzz using `for` instead of `while`
    // `n` will take the values: 1,2, ..., 100 in each iteration
    for n in 1..101 { // notice the 101 on its end
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        }
    }

    println!("=============");

    // alternatively `a..=b` can be used for a range that is inclusive
    // on both ends.
    for n in 1..=100 { // notice the 100 on its end
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        }
    }
}

// for and iterators
pub fn for_and_iterators() {
    // the `for in` construct is able to interact with an Iterator in several ways
    // as discussed in the section on the Iterator trait, by default the `for` loop
    // will apply the `into_iter` function to the collection. However, this is
    // not the only means of converting collections into iterators

    // `into_iter`, `iter`, and `iter_mut` all handle the conversion of a
    // collection into an iterator in different ways, by providing different
    // views on the data within

    // - `iter` this borrows each element of the collection through each
    // iteration. thus leaving the collection untouched and available 
    // for reuse after the loop
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // "Ferris" => println!("There is a rustacean among us!"),
            // try deleting the & and matching just "Ferris"
            // (got compile time error expected &&str, found &str)
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // - `into_iter` this consumes the collection so that on each iteration
    // the exact data is provided, once the collection has been consumed 
    // it is no longer available for reuse as it has been 'moved' to the 
    // loop

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    };

    // println!("names : {:?}", names);
    // ^ compile time error `borrow of moved value: `names`
    // value borrowed here after move`

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names)

    // in the above snippets note the type of `match` branch, that is the
    // key difference in the types of iteration. The difference in type 
    // then of course implies differing actions that are able to be performed
}