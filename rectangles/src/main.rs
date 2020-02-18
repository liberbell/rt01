struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;

    let rect1 = Rectangle {width: 10, height:30};

    println!("Area is {}", area(width, height));

    println!("Area is {}", area2(&rect1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
