struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    let mut user1 = build_user(String::from("email@gmail.com"), String::from("Name"));
    println!("My Name is a {}", user1.username);

    user1.username = String::from("SungJin");
    println!("But it Changed name is a {}", user1.username);

    let _user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
