extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Welcome to the guessing game.")

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("Secret number is = {}", secret_number);

    println!("Please input your guess...");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line.");

    println!("you guessed: {}", guess);
}
