use std::io;
use rand::Rng;

fn main() {
    println!("Hello I'm Bob! Guess the number I'm thinking of! It is between 1 and 100 inclusive.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("My friend Fred will also try to guess. Try to guess it before him!");

    //for loop
    let mut myguess = String::new();

    println!("Your guess:");

    io::stdin().read_line(&mut myguess)
        .expect("Failed to read line");

    let mut fredguess = String::new();

    println!("Fred guessed: {}", fredguess);
}
