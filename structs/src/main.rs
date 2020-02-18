fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_account: u64,
        active: bool,
    };

    let user1 = User {
        email: String::from("example@xyz.com"),
        username: String::from("Taro"),
        active: true,
        sign_in_account: 1,
    };

    println!("{}", user1.email);
    println!("{}", user1.username);
}
