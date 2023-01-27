fn main() {
    println!("Hello, world!");

    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);
    println!("{}", five());
    println!("{}", plus_one(1));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The value of x is: {x}{unit_label}");
}
