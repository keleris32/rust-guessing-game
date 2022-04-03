use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number from 1 - 101 to win!");

    // define a secret number and assign a random value from range 1 - 101
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess:");

    // define an empty string
    let mut guess = String::new();

    // read the standard input entered by the user and assign buffer to guess variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    // trim whitespace from guess and parse it to an integer of type u32
    let guess: u32 = guess.trim().parse().expect("Please type in a number!");

    println!("You guessed: {}", guess);

    // compare the user's guess to the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is too small!"),
        Ordering::Greater => println!("Your guess is too big!"),
        Ordering::Equal => println!("You win! Your guess is correct."),
    }
}
