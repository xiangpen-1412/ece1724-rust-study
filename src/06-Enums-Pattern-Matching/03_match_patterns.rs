// ECE1724 — Lecture 5: Pattern Matching (Book 6.2)
// Pages: 47-54 (Lecture 5 slides; title page = 1).
// Topics: match, value bindings, Option, exhaustiveness, catch-all patterns, _.

// Page 47: keep the first Coin example separate from the state-quarter version.
mod basic {
    pub enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    pub fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // The pattern binds the data inside Quarter to state.
            println!("State quarter from {state:?}!");
            25 // No semicolon: this arm returns a u8, like the other arms.
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    // Removing the None arm causes E0004: matches must cover every variant.
}

fn main() {
    for coin in [
        basic::Coin::Penny,
        basic::Coin::Nickel,
        basic::Coin::Dime,
        basic::Coin::Quarter,
    ] {
        println!("Basic coin: {} cents", basic::value_in_cents(coin));
    }

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value in cents: {}", value_in_cents(coin)); // 25
    for coin in [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
    ] {
        println!("Value in cents: {}", value_in_cents(coin));
    }

    // let five = Some(String::from("hello"));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("plus_one: {five:?} -> {six:?}; None -> {none:?}");

    // A named catch-all binds unmatched values; keep it after specific arms.
    for coin in [Coin::Quarter(UsState::Alaska), Coin::Nickel] {
        match coin {
            Coin::Quarter(state) => println!("Quarter from {state:?}"),
            other_coin => println!("Other coin: {other_coin:?}"),
        }
    }

    // The slide also uses a panicking fallback. This example selects Quarter.
    let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Dime;
    let cents = match coin {
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
        other_coin => panic!("Not a quarter: {other_coin:?}"),
    };
    println!("Quarter-only value: {cents}");

    // _ matches without binding. Both arms return (), so their types agree.
    for coin in [Coin::Quarter(UsState::Alaska), Coin::Penny] {
        match coin {
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
            }
            _ => (), // Ignore every non-quarter.
        }
    }
    // An arm returning 25 and another returning () would be a type error.
}

// Expand (leave for practice):
// TODO: Add a third UsState and use it in a Quarter; check the bound state output.
// TODO: Write plus_two(Option<i32>); Some(5) -> Some(7), None -> None.
