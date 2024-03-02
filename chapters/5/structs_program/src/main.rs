#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("Area of rect1 is {}", area(&rect1));

    // using {:?} we can print the struct with rust's debug formatting
    println!("rect1 is {:?}", rect1);

    // using {:#?} we can print the struct again with rust's debug formatting but have
    // it on multiple lines
    println!("rect1 is {:#?}", rect1);

    // dbg! is a macro that prints the value of the expression along with the file and
    // line number of the macro invocation
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
