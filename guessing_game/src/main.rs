use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Welcome to Guesstimator!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your desired number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Enter a valid Input!");

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You Win!!"),
        }
    }
}
