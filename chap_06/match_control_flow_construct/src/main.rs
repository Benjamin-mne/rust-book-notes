#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(some_coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}