use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=10);
    //println!("The secret number is: {secret_number}");

    loop {
        //infinity loop
        let mut guess = String::new(); //mutable variable guess the type String
        println!("Please input your guess.!:::::");
        io::stdin() //comment
            .read_line(&mut guess)
            .expect("Failed to read line");
        //translate guess from string to u32

        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); -> crashing on error
        //handling the error -> match || crashing on error -> expect
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            //_, is a catch-all value
            Err(_) => {
                println!("wrong input");
                println!("ijshfishf");
                continue;
            }
        };
        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
