fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(10);
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x:i32) {
    println!("The value of x = {}", x);
}
