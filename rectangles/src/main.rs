#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 35,
        height: 60,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 80,
    };

    let square = Rectangle::square(50);

    println!("{}", rect.area());
    println!("{}", square.area());
    println!("{}", rect.can_hold(&rect3));
    println!("{}", rect2.can_hold(&rect));
}
