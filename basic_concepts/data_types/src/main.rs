fn main() {
    // intergers (i(u)8, i(u)16, i(u)31, i(u)64, i(u)128)
    // isize and usize depend on computer program is running on
    let a: i8 = 127;
    let b: u8 = 255;

    // Floating-Point
    let c: f32 = 2.0; // single precision
    let d: f64 = 3.0; // double precision

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    // boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    // character
    // four bytes in size
    let e = 'e';
    let heart_eyed_cat = '😻';

    // Compound Types (tuple and arrays)
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("The value of y is: {}", y);

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
}
