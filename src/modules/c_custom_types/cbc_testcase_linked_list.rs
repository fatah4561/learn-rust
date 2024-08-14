// linked list can be implemented via enums
// (!) i haven't fully grasp this yet
// the are basically using recursive technique
#![allow(dead_code, unused_imports)]
use List::*;

enum List {
    // cons: tuple struct that wraps an element and a pointer to the next
    Cons(u32, Box<List>),
    // Nil: a node that signifies the end of the linked list
    Nil,
}

// attach method to enum
impl List {
    // create an empty list
    fn new() -> List {
        // Nil has type `List`
        Nil
    }

    // consumes a list and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // cons also has type list
        Cons(elem, Box::new(self))
    }

    // append by chatGPT (modify in place)
    // ex: list.append(3);
    fn append(&mut self, elem: u32) {
        match self {
            Cons(_, ref mut tail) => {
                // If we have a `Cons`, we recursively call append on the tail
                tail.append(elem);
            }
            Nil => {
                // If we reach the end of the list (Nil), we replace it with a new Cons
                *self = Cons(elem, Box::new(Nil));
            }
        }
    }

    // custom func like prepend use (returning new list)
    // list = list.appended(3); instead of
    // list.append(3);
    fn appended(self, elem: u32) -> List {
        match self {
            Cons(head, tail) => Cons(head, Box::new(tail.appended(elem))),
            Nil => Cons(elem, Box::new(Nil)),
        }
    }

    // return the length of the list
    fn len(&self) -> u32 {
        // self has to be matched because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching
        // concrete type `T` is preferred over a match on a reference `&T`
        // after rust 2018 can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        match *self {
            // can't take ownership of the tail, because `self` is borrowed
            // instead take a reference to the tail
            // my summary use ref because moving tail the original self
            // no longer contains the full list, which is not allowed
            // (still dk what happens if doing so with the len result)
            Cons(_, ref tail) => 1 + tail.len(),
            // Base case: an empty list has zero length
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!` but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn cbc_testcase_linked_list() {
    let mut list = List::new();

    // prepend some elements
    // ex: 2 (2) <- 1 (1) <- nil (0)
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // show the final state of the list
    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());

    // print after append
    list.append(4);
    list.append(5);
    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());

    // another one
    list = list.appended(6);
    list = list.appended(7);

    println!("linked list has length {}", list.len());
    println!("{}", list.stringify());
}
