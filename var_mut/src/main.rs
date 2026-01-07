fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value is y is: {y}");

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("The values of t and f are: {t} ander {f}");

}
