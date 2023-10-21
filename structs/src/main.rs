#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let mut u1 = User {
        active: true,
        username: String::from("James"),
        email: String::from("james@hotmail.com"),
        sign_in_count: 1,
    };
    u1.email = String::from("notjames@htomail.com");

    println!("This is user: {u1:?}");

    let u2 = build_user(String::from("mariusz@hotmail.com"), String::from("mariusz"));
    println!("This is user 2: {u2:?}");

    let u3 = User {
        email: String::from("mariusz_2@hotmail.com"),
        ..u2
    };
    println!("This is user 3: {u3:?}");

    let c = Color(0, 0, 0);
    println!("This is color {c:?}");
    let p = Point(0, 0, 0);
    println!("This is point {p:?}");

    let eq = AlwaysEqual;
    println!("This is always equal struct: {eq:?}")
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
