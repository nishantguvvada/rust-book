fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // if we don't add : u32 type annotation, Rust will display the 'no_type_annotations' error
    println!("The value of guess: {guess}");
}
