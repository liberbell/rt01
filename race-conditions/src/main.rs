fn main() {
    let refer_nothing = dangle;

    println!("{}", refer_nothing);
}

fn dungle() -> &String {
    let s = String::from("hello")

    &s;
}
