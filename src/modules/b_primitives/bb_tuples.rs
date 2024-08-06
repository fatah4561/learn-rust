use std::fmt;


// reverse tuple (pair is var name)
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    // return (bool_param, int_param);

    // if not using ; then will act as return statement
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

// ACTIVITY
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({a:} {b:}) \n({c:} {d:})",
            a = self.0,
            b = self.1,
            c = self.2,
            d = self.3,
        )
    }
}

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

#[allow(dead_code, unused_imports)]
pub fn bb_tuples() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // extract tuple values using tuple indexing
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // tuple as tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // tuple with more than (>12) 12 elements can't be printed
    // (for reasons why just google it)
    // let too_long_tuple = (1,2,3,4,5,6,7,8,9,10,11,12,13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Reversed pair is {:?}", reverse(pair));

    // destructured tuple to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    // struct 
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    
    println!("{:}", matrix);
    println!("Transposed: \n{:}", transpose(matrix));

}
