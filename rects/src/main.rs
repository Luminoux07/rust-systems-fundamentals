#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        _height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero {}", rect1.width);
    }
}
