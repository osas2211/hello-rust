use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    rust_compound_types();
}

#[allow(unused_variables)]
fn rust_compound_types() {
    let tuple: (&str, i8, u8) = ("Let's go", 1, 240);
    let arr: [u8; 7] = [5; 7];
    println!("{}", tuple.0);
    println!("{}", arr[0])
}

#[allow(dead_code)]
fn guessing_game() {
    loop {
        println!("Enter your guess: ");
        let mut guess: String = String::new();
        let secret_number: u8 = rand::thread_rng().gen_range(1..=5);

        match stdin().read_line(&mut guess) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let guess: u8 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Guess {guess} -- Secret Number {secret_number}");
    }
}
