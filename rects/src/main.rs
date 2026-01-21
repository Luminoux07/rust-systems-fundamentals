#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    length: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 50,
        length: 20,
    };

    println!("The area is {} pixels", area(&rect1)
        );
    
    println!("The rectangle be like: {rect1:#?}")
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height / rectangle.length
}

