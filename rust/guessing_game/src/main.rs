use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to LovetheFrog's Rust guessing game!");
    println!("--------------------------------------------");

    let number_to_guess = rand::thread_rng().gen_range(1..=100);

    println!();
    println!();

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was {guess}");

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Guess is too small!"),
            Ordering::Greater => println!("Guess is too big!"),
            Ordering::Equal => {
                println!("Guess is correct!");
                break;
            }
        }
    }
}
