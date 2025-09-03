fn main() {
    let mut s = String::from("Hello"); // bind hello value to s
    s.push_str(", World");
    println!("{s}");

    s = String::from("ahoy"); // bind ahoy value to s, hello value is freed right away
    println!("{s}, world!");
}
