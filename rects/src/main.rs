#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("The size is of {} pixels.",
            area(&rect1));
    println!("This is size {rect1:#?}");
    
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

