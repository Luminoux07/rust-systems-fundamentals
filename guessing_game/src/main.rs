use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("🎲 Welcome to the Number Guessing Game!");
    println!("=======================================");
    
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    
    println!("\nI've picked a number between 1 and 100.");
    println!("Can you guess what it is?\n");
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        
        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Parse the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("⚠️  Please enter a number between 1 and 100!\n");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("⚠️  Please enter a valid number!\n");
                continue;
            }
        };
        
        attempts += 1;
        
        println!("You guessed: {}", guess);
        
        // Compare guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small! Try a bigger number.\n"),
            Ordering::Greater => println!("📈 Too big! Try a smaller number.\n"),
            Ordering::Equal => {
                println!("\n🎉 Congratulations! You guessed the correct number!");
                println!("🏆 You won in {} attempt{}!", 
                    attempts, 
                    if attempts == 1 { "" } else { "s" }
                );
                
                // Performance feedback
                match attempts {
                    1 => println!("🌟 Incredible! You got it in one try!"),
                    2..=5 => println!("🌟 Excellent guessing!"),
                    6..=10 => println!("👍 Good job!"),
                    _ => println!("💪 You made it!"),
                }
                
                break;
            }
        }
    }
    
    println!("\nThanks for playing! 👋");
}
