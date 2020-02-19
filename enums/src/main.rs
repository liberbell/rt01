enum IpAdrKind {
    v4(String),
    v6(String),
}

fn main() {
    let four = IpAdrKind::V4;
    let six = IpAdrKind::V6;
}
