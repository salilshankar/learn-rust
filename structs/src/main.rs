struct Rectangle {
    length: u32, 
    width: u32,
}

fn main() {
    let rect = Rectangle {
        length: 1,
        width: 2,
    };

    println!("area of rectangle: {}", area(&rect))
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}