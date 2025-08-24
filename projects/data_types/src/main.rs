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
}
