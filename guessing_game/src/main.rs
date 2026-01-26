use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

fn main() {
    loop {
        println!("🎯 Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut attempts = 0;
        let start_time = Instant::now();

        loop {
            if start_time.elapsed().as_secs() > 30 {
                println!("⏰ Time's up! The number was {secret_number}");
                break;
            }

            println!("Enter your guess:");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number!");
                    continue;
                }
            };

            attempts += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("🎉 You win!");
                    println!("Attempts: {attempts}");
                    println!("Time taken: {} seconds", start_time.elapsed().as_secs());
                    break;
                }
            }
        }

        println!("Play again? (y/n)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.trim().to_lowercase() != "y" {
            break;
        }
    }
}

