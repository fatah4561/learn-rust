use ferris_says::say;
use std::fmt::{self, Display, Formatter};
use std::io::{stdout, BufWriter};

struct ImplDisplayStructure(i32);

impl fmt::Display for ImplDisplayStructure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implement display
#[derive(Debug)]
struct MinMax(i32, i32);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_bits = self.x.to_bits();
        let y_bits = self.y.to_bits();
        write!(f, "{}, {}", x_bits, y_bits)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// 1.2.2.1
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

// 1.2.3
struct City {
    name: &'static str, // lifetime infinite until end of program, use case is for global constant, etc
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}: {:3}°{} {:3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rgb = (self.red as i32 * 65536) + (self.green as i32 * 256) + self.blue as i32;
        let rgb_hex = format!("{:X}", rgb);
        write!(
            f,
            "RGB ({r:}, {g:}, {b:}) 0x{rgb:0>6}",
            r = self.red,
            g = self.green,
            b = self.blue,
            rgb = rgb_hex
        )
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("What does Point2D look like in binary: {:b}?", point);

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // 1.2.2.1
    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    //  1.3
    for city in [
        City {
            name: "Dublin",
            lat: 54.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city)
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color)
    }

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
