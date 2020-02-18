fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let user1 = User {
        email: String::from("example@xyz.com"),
        username: String::from("Taro"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);

    let user2 = User {
        email: String::from("user2@abc.com");
        username: String::from("user2");
    }
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}
