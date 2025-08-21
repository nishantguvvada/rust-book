use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng() // local to the current thread of execution and is seeded by operating system
        .gen_range(1..=100); // defined in Rng trait, takes a range expression and generates a random number in the range

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable named guess, String::new: is a function that returns a new instance of a String
                                       // creates a new empty string

        io::stdin() // stdin function returns an instance of std::io::Stdin which is a type representing a handle to standard input for terminal
            .read_line(&mut guess) // calls the read_line method on standard input handle, returns a Result value
            .expect("Failed to read line"); //  An instance of Result has an expect method,  If this instance of Result is an Err value, expect will cause the program to crash and display the message

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // tells the program to go to the next iteration of the loop
        } // parse method on strings converts a string to another type

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
