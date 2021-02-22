mod quad;

use quad::{rectangle, square};

fn main() {
    let rect = rectangle::Rect {
        length: 5,
        width: 3,
    };

    let sq = square::square(5);

    println!("rectangle area: {}", rect.area());
    println!("square area: {}", sq.area());
}
