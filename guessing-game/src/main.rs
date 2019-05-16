use std::io;

fn main() {
    println!("Hello I'm Bob! Guess the number I'm thinking of! It is anything from 1 to 100.");
    print!("My friend Fred will also try to guess. Try to guess it before him!");

    let mut myguess = String::new();

    io::stdin().read_line(&mut myguess)
        .expect("Failed to read line");

    println!("You guessed: {}", myguess);
}
