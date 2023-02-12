fn main() {
    let coins: [Coin; 3] = [
        Coin::Penny,
        Coin::Quarter(UsState::Alabama),
        Coin::FooCoin];

    for coin in coins {
        let mut coin_value = value_in_cents(&coin);
        print_value(coin_value, &coin);
    }
}

fn print_value(coin_value: Option<u8>, coin: &Coin) -> () {
    if let Some(value) = coin_value {
        println!("{:?} is {:?} cents", coin, value);
    } else {
        println!("{:?} isn't worth any cents!", coin);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    FooCoin,
}

fn value_in_cents(coin: &Coin) -> Option<u8> {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            Some(1)
        }
        Coin::Nickel => Some(5),
        Coin::Dime => Some(10),
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            Some(25)
        }
        _ => None
    }
}