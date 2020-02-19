enum IpAddrKind {
    v4(String),
    v6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"));
}
