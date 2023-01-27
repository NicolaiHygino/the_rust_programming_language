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
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q_alabama = Coin::Quarter(UsState::Alabama);
    let q_alaska = Coin::Quarter(UsState::Alaska);

    value_in_cents(p);
    value_in_cents(n);
    value_in_cents(d);
    value_in_cents(q_alabama);
    value_in_cents(q_alaska);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
