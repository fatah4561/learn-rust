#![allow(dead_code, unused_imports)]

/*
    some functions are connected to a particular type, these come in 2 forms
    associated functions and methods. Associated functions are functions that 
    are defined on a type generally, while methods are associated functions
    that are called on a particular instance of a type (the object itself not
    the class)
*/

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// implementation block, all `Point` associated functions & methods
// will go in here

impl Point { 
    // this is an associated function because this function is associated 
    // to a particular type, that is, Point.
    // 
    // associated functions don't need to be called with an instance
    // these functions are generally used like constructors
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // another associated function, taking 2 arguments:
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // this is a method
    // `&self` is sugar for `self: &self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`

    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the 
        // caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        2.0 * ((x1-x2).abs() + (y1-y2).abs())
    }

    // this method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self` as above
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: 2 heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // this method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    fn destroy(self) {
        // destructures self
        // using self will move the object the `self` refer
        // to here so it is destroyed outside
        let Pair(first, second) = self;
        println!("destroying Pair({}, {})", first, second);
        // first and second go out of scope and get cleaned up
    }
}

pub fn ia_methods() {
    let rectangle = Rectangle {
        // associated functions are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // methods are called using the dot operator
    // note that the first argument `&self` is implicitly passed, 
    // ie `rectangle.perimeter() === rectangle.perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // error `rectangle` is immutable, but this method requires a mutable
    // object
    // rectangle.translate(1.0, 0.0);
    // ^uncomment to see compiler time error

    // mutable objects can call mutable methods
    square.translate(1.0, 1.0);
    println!("Resulting square {:?}", square);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // error previous `destroy` call "consumed" `pair`
    // pair.destroy();
    // ^uncomment to see compiler time error
    // (use of moved value)
}
