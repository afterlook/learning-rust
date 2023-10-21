#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // x.map(|x| x + 1)
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}

fn main() {
    // Simple pattern matching with enum value unwrapping
    println!("Value of penny in cents is {}", value_in_cents(Coin::Penny));
    println!(
        "Value of nickel in cents is {}",
        value_in_cents(Coin::Nickel)
    );
    println!("Value of dime in cents is {}", value_in_cents(Coin::Dime));
    println!(
        "Value of quarter in cents is {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!(
        "Value of quarter in cents is {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    // Checking for none
    let five = Some(5);
    let six = plus_one(five);
    if let Some(x) = six {
        println!("This is option six: {}", x)
    }
    let none = plus_one(None);
    if none.is_none() {
        println!("This is none");
    }

    // match catch all
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("Fancy hat");
}
fn remove_fancy_hat() {
    println!("Remove fancy hat");
}
fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces", num_spaces);
}
fn reroll() {
    println!("Rerolling ");
}
