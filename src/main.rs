use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number from 1 - 5 to win!");

    // define a secret number and assign a random value from range 1 - 5
    let secret_number = rand::thread_rng().gen_range(1, 5);

    loop {
        println!("{}", "Input your guess:".blue());

        // define an empty string
        let mut guess = String::new();

        // read the standard input entered by the user and assign buffer to guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // trim whitespace from guess and parse it to an integer of type u32
        // compare the result gotten while trying to parse the user's input
        // if the parse is successful then return the parsed values
        // if it isn't then print to the screen and continue to the next iteration!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Warning: Please enter a number!".red());
                continue;
            }
        };

        // compare the user's guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Your guess is too small!".red()),
            Ordering::Greater => println!("{}", "Your guess is too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win! Your guess is correct.".green());
                println!("The secret number was: {}", secret_number);
                break;
            }
        }
    }
}
