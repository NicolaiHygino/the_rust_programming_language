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
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("{}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("{:?}", state),
        _ => count += 1,
    }
}
