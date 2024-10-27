use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    loop {
        println!("Guess the game!");
        println!("Enter your Guess: ");
        let secret_number: u8 = rand::thread_rng().gen_range(1..=5);
        let mut guess: String = String::new();
        match io::stdin().read_line(&mut guess) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let guess: u8 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
