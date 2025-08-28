fn main() {
    // let number = 3;

    // if number < 1 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // if number { // throws error
    //     println!("number was three");
    // }

    // if number != 0 {
    //     println!("number was something other than zero");
    // }

    // if number % 4 == 0 {
    //     println!("divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("divisible by 3");
    // } else {
    //     println!("number is not divisible by 3 or 4");
    // }

    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
