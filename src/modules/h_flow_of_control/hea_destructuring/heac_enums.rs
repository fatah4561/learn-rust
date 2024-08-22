#![allow(dead_code, unused_imports)]

enum Color {
    // these 3 are specified solely by their name
    Red,
    Blue,
    Green,
    // these likewise tie `u32` tuples to different names: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn heac_enums() {
    // an `enum` is destructured similarly
    // personal tips on enum vs struct
    // in enum we variable bind it choosing single variant ex let red = Color::Red()
    // while in struct we create an struct (or object) and use all of the fields in the
    // struct ex let person = Person {name: String::from("f"), age: 22}

    let color = Color::RGB(122, 17, 40); // rgb
    let color = Color::Red; // red
    let color = Color::Blue; // blue
    let color = Color::Green; // green
    let color = Color::CMY(30, 30, 30); // cmy
    let color = Color::CMYK(30, 30, 30, 30); // cmyk
    let color = Color::HSV(303, 303, 303); // hsv
    // ^ try different variants of color

    println!("what color is it?");
    // an enum cna be destructured using a match
    match color {
        Color::Red => println!("The color is red!"),
        Color::Blue => println!("The color is blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("cyan: {}, magenta: {}, yellow: {}, key (black): {}", c, m, y, k),
        // don't need another arm because all variants have been examined ex _ catch all
        // (vs code tips use cmd+ . to fill arms)
    }

}