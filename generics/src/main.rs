struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = list.get(0).expect("To have at least one item");

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    let p1 = Point { x: 5, y: 10.0 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // let number_list = vec![34, 50, 25, 100, 65];
    // println!("{}", largest(&number_list));

    // let number_list2 = vec![344, 500, 425, 3213, 655];
    // println!("{}", largest(&number_list2));

    // let char_list = vec!['n', 'i', 'c', 'o', 'l', 'a', 'i'];
    // println!("{}", largest(&char_list));
}
