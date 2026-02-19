fn main() {
    println!("Hello, world!");

    fun_func(40);
    another_function();
    print_labeled_measurements(5, 'h');
    
}

fn another_function() {
    println!("Another sweet function.");
}

fn fun_func(x: i32) {
    println!("Let's just observe this: {x} times!");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


/*
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guess Game!");
    println!("======================");

    let secret_number = rand::thread_rng()..gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io.stdin()
            .read_line(& mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You've guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
        }
    }
}
*/
