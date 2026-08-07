use std::{cmp::Ordering, io};

use rand;

fn main() {
    tracing_subscriber::fmt::init();
    
    let secret_number = rand::random_range(1..=100);
    tracing::info!("Guess the integer!");    

    loop {
        tracing::info!("Input your guess:");
        let mut guess_number = String::new();

        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read input");
        let guess: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                tracing::info!("Input must be an integer");
                continue;
            }
        };
        tracing::info!("You guessed: {guess_number}");

        // Internal ordering cmp
        match guess.cmp(&secret_number) {
            Ordering::Less => tracing::info!("Too small!"),
            Ordering::Greater => tracing::info!("Too big!"),
            Ordering::Equal => {
                tracing::info!("You win!");
                break;
            }
        }
    }
}
