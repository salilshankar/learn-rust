#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Penny is {} cents", value_in_cents(Coin::Penny));
    println!("Dime is {} cents", value_in_cents(Coin::Dime));
    println!("Nickel is {} cents", value_in_cents(Coin::Nickel));
    println!(
        "Quarter is {} cents",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!(
        "Quarter is {} cents",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("With love from {:?}", state);
            25
        }
    }
}
