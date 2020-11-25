use std::io;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    println!("Please input your guess (1 - 100).");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    let secret_number = rand::thread_rng().gen_range(1, 100)

    println!("You guessed: {}", guess);
}
