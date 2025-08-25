fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // if we don't add : u32 type annotation, Rust will display the 'no_type_annotations' error
    println!("The value of guess: {guess}");

    // Floating-Point Types
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The x and y values are: {x}, {y}");

    // Numeric Operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 5;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // Boolean Types
    let t = true;
    let f: bool = false;
    println!("The value of t is {t} and f is {f}");

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is {y}");

    let five_hundred = tup.0;

    println!("The value of five hundred is {five_hundred}");

    // Array Type

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [2; 4];

    let a_0 = a[0];
    let b_0 = b[0];

    println!("The value of a[0] is {a_0} and b[0] is {b_0}");
}
