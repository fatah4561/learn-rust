#![allow(dead_code, unused_imports)]

// some notes to self
// Option<> is just a type of enum which contain Some() or None()
// some means it has value while none is empty (obvsly none)
// rust doesn't have null value so this can be used
// when function return empty or not
pub fn hf_if_let() {
    // for some use cases, when matching enums, `match` keyword is akward
    // ex:
    let optional = Some(7);
    match  optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^needed 2 indentations just so we could destructure
            // `i` from the option
        },
        _ => {},
        // ^required because `match` is exhaustive. doesn't it seem
        // like wasted space?
    }

    // `if let` is cleaner for this use case and in addition allows various
    // failure options to be specified:

    // all have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads : "if `let` destructures `number`
    // into `Some(i)` evaluate the block (`{}`)"
    if let Some(i) = number {
        println!("matched {:?}", i);
    }

    // if you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("matched {:?}", i);
    } else {
        // destructure failed, change to the failure case
        println!("didn't match a number. let's go with a letter");
    }

    // provide an altered failing condition
    let i_like_letters = true;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
        // destructure failed evaluate an `else if` condition to see if the
        // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("didn't match a number. let's go with a letter!");
    } else {
        // the condition evaluated to false, this branch is the default:
        println!("i don't like letters. let's go with an emoticon :)");
    }

    // in the same way, `ife let` can be used to match any enum value

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // examples
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is a foobar");
    }

    // variable b does not match Foo::Bar
    // so this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // variable c matches Foo::Qux which has a value
    // similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value)
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 120) = c { // if c.value==120
        println!("c is one hundred twenty {}", value)
    }

    // another benefit is that `if let` allows us to match non-parameterized
    // enum variants, this is true even in cases where the enum doesn't
    // implement or derive PartialEq, in such case if Foo::Bar == a would fail to
    // compile, because instances of enum cannot be equated, however 
    // `if let` will continue to work

    //  CHALLENGE
    // this enum purposely neither implements nor derives PartialEq
    // that is why comparing Foo::Bar == a fails below
    enum Foo2 {Bar}

    let a2 = Foo2::Bar;

    // variable a matches Foo2::Bar
    // if Foo2::Bar == a {
    if let Foo2::Bar = a2 {
        // ^this causes a compile-time error, use `if let` instead
        println!("a is a foobar"); 
    }
}