#[derive(Debug)]
enum UsState {
    North,
    South,
    East,
    West,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::North));
}
