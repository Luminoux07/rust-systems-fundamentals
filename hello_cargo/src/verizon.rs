use std::io;

fn main(){
    println!("This is a guessing game!");

    println!("Enter your guess: ");

    let mut guesst = String::new();

    io::stdin()
        .read_line(&mut guesst)
        .expect("Failed to read line");

    println!("Your guesst is: {guesst}");

}
