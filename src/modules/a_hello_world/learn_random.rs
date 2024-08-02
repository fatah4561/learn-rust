use std::{
    fmt,
    io::{stdout, BufWriter},
};

use ferris_says::say;

struct ImplDisplayStructure(i32);

impl fmt::Display for ImplDisplayStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[allow(dead_code)]
pub fn learn_random() {
    // learn randoms:
    {
        let s = String::from("Hello w");
        println!("{}", s)
    }
    let stdout = stdout();
    let message = String::from("Hello World");
    let mut width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    println!("Hello, world!");

    width = 100;
    println!("{} days", width);
    println!("{0}, this is {1}. {1}, this is {0}", "alice", "bob");
    println!(
        "{subject} {verb} {object}",
        subject = "he",
        object = "the hotdog",
        verb = "ate"
    );

    // different formatting
    println!("Base 10: {}", 7000);
    println!("Base 2 (binary): {:b}", 7000);
    println!("Base 8 (octal): {:o}", 7000);
    println!("Base 16 (hexadecimal): {:x}", 7000);

    println!("{number:>5}", number = 1);
    println!("{number:0>5}", number = 1);
    println!("{number:0<5}", number = 1);
    println!("{number:8>width$}", number = "ah", width = 30);
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // struct Structure(i32);
    // fmt::Display;
    // println!("This struct {} won't print...", Structure(90))

    let it: f64 = 1.9999;
    let out: usize = 5;
    println!("{it:>out$}");

    let pi = 3.141592;
    println!("{pi:.3}");

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "slater",
        "muslim",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "peter";
    let age = 28;
    let peter = Person { name, age };

    println!("{:#?}", peter);
    println!("{:#?}", peter.age);

    let imp_display = ImplDisplayStructure(9);
    println!("{}", imp_display)
}
