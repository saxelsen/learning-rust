extern crate rand;

use std::io::{self, Write};
use std::cmp::Ordering;
use std::__rand::thread_rng;

fn main() {
    println!("Guess the number");

    let mut secret_number = __rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
 	io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
}
