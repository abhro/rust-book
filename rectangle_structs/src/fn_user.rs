fn main() {
    let rect1 = Rectangle { width: 30, length: 50 };

    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    length: u32,
}
