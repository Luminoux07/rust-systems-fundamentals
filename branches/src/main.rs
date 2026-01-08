fn main() {
   let number = 9;

   if number < 3 {
        println!("condition was true");
   } else {
        println!("condition was false");
   }

   if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;
    let number = if condition { 3 } else { 7 };

    println!("The value of number is: {number}");
}
