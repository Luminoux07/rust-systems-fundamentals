use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welocme to Guess It");
    println("=====================");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

    }
}
