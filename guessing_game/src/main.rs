use std::io;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess (1 - 100).");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // we recieve input as a String, conversion enables comparison w/ integers 
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You Win!");
                break;
            }
        }
    }
}
