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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {
    let coin1 = value_in_cents(Coin::Quarter(UsState::Alaska));
    let coin1_5 = value_in_cents(Coin::Quarter(UsState::Alabama));
    let coin2 = value_in_cents(Coin::Penny);
    let coin3 = value_in_cents(Coin::Nickel);
    let coin4 = value_in_cents(Coin::Dime);
    println!("Coin: {:?}", coin1);
    println!("Coin: {:?}", coin1_5);
    println!("Coin: {:?}", coin2);
    println!("Coin: {:?}", coin3);
    println!("Coin: {:?}", coin4);
}
