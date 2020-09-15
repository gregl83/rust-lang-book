#[derive(Debug)]
enum UsState {
    Florida,
    Texas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State: {:?}", state);
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

fn main() {
    // match enum by variant
    println!("Cents in Dime: {}", value_in_cents(Coin::Dime));

    // match enum by variant of type
    println!("Cents in Quarter: {}", value_in_cents(Coin::Quarter(UsState::Florida)));

    // match enum option for some action
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!(
        "five: {:?} six: {:?} none: {:?}",
        five,
        six,
        none
    );

    // match _ placeholder
    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
