use rand::Rng;
use std::io;

use crate::input::input;

pub fn random() {
    let answer = rand::rng().random_range(1..=100);

    for i in 0..3 {
        let input = input("Enter A Guess", i);

        match input {
            0 => {
                println!("No valid input provided.");
            }
            n if n < answer && n > (answer / 2) => {
                println!("Too small");
            }
            n if n < answer => {
                println!("Way too small");
            }
            n if n > answer && n < (answer * 2) => {
                println!("Too large");
            }
            n if n > answer => {
                println!("Way too large");
            }
            _ => {
                println!("Congratulations! You guessed the number.");
                return;
            }
        }
    }

    println!("Sorry, you've used all your attempts. The correct number was: {}", answer);
}
