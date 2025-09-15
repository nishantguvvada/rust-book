enum Message {
    Quit,                       // Has no data associated with it at all
    Move { x: i32, y: i32 },    // Has named fields, like a struct does
    Write(String),              // Includes a single String
    ChangeColor(i32, i32, i32), // Includes three i32 values
}

fn main() {
    println!("Hello, world!");
}
