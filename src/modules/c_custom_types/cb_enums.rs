// enums a type that has many variants
#![allow(dead_code)]

enum WebEvent {
    // unit-like
    PageLoad,
    PageUnload,
    // tuple
    KeyPress(char),
    Paste(String),
    // or c-like struct
    Click{x: i64, y: i64}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x,y)
        },
    }
}

// type aliases
// useful if enum name too long or too generic (renaming)
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    // this can be used to store func like ability (see the impl below)
    Add, 
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; // here

// commonly being used in impl block using the `self` alias:
impl VeryVerboseEnumOfThingsToDoWithNumbers{
    fn run(&self, x: i32, y:i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            // VeryVerboseEnumOfThingsToDoWithNumbers::Add => todo!(),
            // VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => todo!(),
            
        }
    }
    fn paw(&self) {
        println!("paw bro")
    }
}
pub fn cb_enums() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned() creates an owned `String` from a string slice.
    // let s: &str = "a";
    // let ss: String = s.to_owned();
    let pasted = WebEvent::Paste("my text".to_owned());
    // let pasted = WebEvent::Paste(String::from("my text").to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // TYPE ALIASES
    // referring to each variant via its alias
    let x = Operations::Add;

    // use
    let addition = Operations::Add;
    let subtraction = Operations::Subtract;

    let result_add = addition.run(5, 3);
    let result_sub = subtraction.run(5, 3);

    println!("Addition result {}", result_add);
    println!("Subtraction result {}",result_sub);
    addition.paw();
}