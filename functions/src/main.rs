fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(10);

    let a = sum(10, 20);
    println!("sum = {}", a);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x:i32) {
    println!("The value of x = {}", x);
}

fn sum(x:i32, y:i32) -> i32 {
    let b = x + y;
    b * 100;
}
