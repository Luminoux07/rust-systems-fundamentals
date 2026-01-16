use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Welcome to Guest Season!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess here: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = guess.trim().parse().expect("Enter a valid number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win Man!!");
                break;
            }
        }
    }
}
