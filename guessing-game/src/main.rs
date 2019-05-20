use std::io;
use std::cmp::Ordering;
// use std::io::{self, Write};
use rand::Rng;

fn get_ai_guess(min: u32, max: u32) -> u32 {
    // AI's guess
    return min + ((max-min)/2);
}

fn main() {
    let debug = true;

    println!("Hello I'm Bob! Guess the number I'm thinking of! It is between 1 and 100 inclusive.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    if debug {
        println!("The secret number is: {}", secret_number);
    }  

    println!("My friend Fred will also try to guess. Try to guess it before him!");

    let mut min: u32 = 1;
    let mut max: u32 = 100;

    loop {
        println!("Please input your guess.");
        //TODO: Fix input to be on same line
        // print!("Your guess: ");
        // io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        let aiguess = get_ai_guess(min,max);

        println!("Fred guessed: {}", aiguess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Your guess is too small!");
                if guess>min {
                    min = guess;
                }
            },
            Ordering::Greater => {
                println!("Your guess is too big!");
                if guess<max {
                    max = guess;
                }
            },
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }

        match aiguess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Fred's guess is too small!");
                if aiguess>min {
                    min = aiguess;
                }
            },
            Ordering::Greater => {
                println!("Fred's guess is too big!");
                if aiguess<max {
                    max = aiguess;
                }
            },
            Ordering::Equal => {
                println!("Fred won!");
                break;
            }
        }
    }
}