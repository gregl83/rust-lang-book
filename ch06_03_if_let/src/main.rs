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

fn main() {
    // match only one condition
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let to match single condition
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // match if else
    let state = UsState::Florida;
    let coin = Coin::Quarter(state);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // if else
    let state = UsState::Florida;
    let coin = Coin::Quarter(state);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
