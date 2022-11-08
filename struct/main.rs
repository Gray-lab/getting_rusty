struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1: User = build_user(String::from("martin.g@email.com"), String::from("mg"));
    let user2 = User{
        email: String::from("cat.w@email.com"),
        username: String::from("cw"),
        ..user1
    };
    println!("{},{}", user1.email, user2.email);
    println!("{},{}", user1.username, user2.username);
    println!("{},{}", user1.active, user2.active);
    println!("{},{}", user1.sign_in_count, user2.sign_in_count);
}