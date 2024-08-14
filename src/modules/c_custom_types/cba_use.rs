// use declaration can be used so manual scoping isn't need
#![allow(dead_code)]

enum Stage {
    Beginner,
    Intermediate,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

pub fn cba_use() {
    //  explicitly use each name so they are available without manual scoping
    // manual scoping `let stage = Stage::Beginner;``
    // if using crate then move the enums to main module
    // use crate::Stage::{Beginner, Intermediate, Advanced};
    use Stage::{Beginner, Intermediate, Advanced};
    // automatically use each name
    use Role::*;

    // equivalent to `Stage::Beginner`
    let stage = Beginner;
    let role = Student;

    // note the lack of scoping
    match stage {
        Beginner => println!("Beginner"),
        Intermediate => println!("Intermediate"),
        Advanced => println!("Advanced"),
    }

    match role {
        Student => println!("Student"),
        Teacher => println!("Teacher"),
    }

    // using scoping
    // match stage {
    //     Stage::Beginner => println!("Beginner"),
    //     Stage::Intermediate => println!("Intermediate"),
    //     Stage::Advanced => println!("Advanced"),
    // }

    // match role {
    //     Role::Student => println!("Student"),
    //     Role::Teacher => println!("Teacher"),
    // }
}