enum IpAddrKind {
    v4(String),
    v6(String),
}

enum Message {
    Quit,
    Move (X:i32, Y:i32),
    Write(String)
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}
