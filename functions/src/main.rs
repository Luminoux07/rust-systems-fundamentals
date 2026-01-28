fn main() {
    println!("Hello, world!");

    fun_func(50);
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
