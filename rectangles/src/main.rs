fn main() {
    let width = 30;
    let height = 50;

    println!("Area is {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
