use rand::Rng;
use std::io;
use std::io::Ordering
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.!:::::");

    let mut guess = String::new(); //mutable variable guess the type String

    io::stdin() //comment
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}");
}
