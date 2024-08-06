// tuple struct (just a named tuples)
struct Pair(i32, f32);

// unit struct, field_less useful for generics
struct Unit;

// field struct (classic C struct)
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// also field struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// reusing structure as field
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// ACTIVITY
fn rect_area(r: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = r;

    let height = x2 - x1;
    let width = y2 - y1;

    height * width
}

fn square(point: Point, float: f32) -> Rectangle {
    // x1, y1
    // x2?, y2?
    // if height = x2 - x1
    // then float = x2 - point.x
    // ex 3.3 = x2 - 4
    // then 3.3 + 4 = x2
    // summary we just need to add it

    Rectangle {
        top_left: Point{
            x: point.x,
            y: point.y,
        },
        bottom_right: Point {
            x: point.x + float,
            y: point.y + float,
        },
    }
}

#[allow(dead_code, unused_imports)]
pub fn ca_structures() {
    // create struct with field init shorthand (happens if both var name equal)
    let name = String::from("Peter Parker");
    let age = 27;
    let peter = Person { age, name };

    // uncomment to test
    // let ages = 27;
    // let peter = Person{ages, name};

    println!("{:?}", peter);

    // instantiate a `point`
    let point = Point { x: 10.3, y: 0.4 };
    let another_point = Point { x: 5.2, y: 0.2 };

    // access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make another struct by using struct update syntax to use the field
    // of another struct
    let bottom_right = Point {
        x: 5.2,
        ..another_point
    };

    // bottom_right.y will be the same as another_point.y
    println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point using let binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    println!("Destructured left is {}, top is {}", left_edge, top_edge);

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right, // or bottom_right: bottom_right
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    // access tuple struct field
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("Area is {}", rect_area(rectangle));

    let rec2 = square(point, 3.333);
    println!("Wow {:?}", rec2)
}
