fn main() {
    let rect: (i32, i32) = (5,4); 

    let (l, b) = rect;

    let area = l * b;

    println!("area of rect: {}", area);
}
