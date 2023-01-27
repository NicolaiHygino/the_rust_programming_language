struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: i64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("nicola@lindo.com"),
        username: String::from("nicolailindo"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.username);

    let user2 = build_user(String::from("nicolai"), String::from("nicolailindo"));
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("valkiria_feia"),
        ..user1
    };

    let black = Color(0, 1, 0);
    let origin = Point(0, 9, 0);

    println!("{}", black.1);
    println!("{}", origin.1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
