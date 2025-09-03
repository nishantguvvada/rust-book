fn main() {
    let mut s = String::from("Hello"); // bind hello value to s
    s.push_str(", World");
    println!("{s}");

    s = String::from("ahoy"); // bind ahoy value to s, hello value is freed right away
    println!("{s}, world!");

    let a = String::from("yes"); // a comes into scope

    takes_ownership(a); // a's value moves into the function, a is no longer valid from here

    // println!("{a}"); // This line will throw error 'borrow of moved value: `a`'

    let x = 5;

    makes_copy(x); // x is i32 that implements the Copy trait (stored onto the stack), x does not move into the function, can be used afterward

    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{s1}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("Some String {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("Some Integer {some_integer}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
