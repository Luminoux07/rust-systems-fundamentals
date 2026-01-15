fn main() {
    
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    let b1 = String::from("aloha");

    let len = calculate_breadth(&b1);

    println!("The breadth of {b1} is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_breadth(b: &String) -> usize {
    b.len()
}
