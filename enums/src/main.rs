enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {X:i32, Y:i32},
    Write(String)
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("I`m inside Call!");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"))
}
