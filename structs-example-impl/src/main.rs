#[derive(Debug)]
struct Quadrilateral {
    length: u32,
    width: u32,
}

impl Quadrilateral {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Quadrilateral) -> String {
        match self.length < other.length && self.width < other.width {
            true => return String::from("No"),
            false => return String::from("Yes"),
        }
    }

    fn create_square(edge: u32) -> Quadrilateral {
        Quadrilateral {
            length: edge,
            width: edge,
        }
    }

    fn create_rectangle(length: u32, width: u32) -> Quadrilateral {
        Quadrilateral {
            length: length,
            width: width,
        }
    }
}

fn main() {
    let rect = Quadrilateral {
        length: 30,
        width: 40,
    };

    let rect2 = Quadrilateral::create_rectangle(10, 20);

    let rect3 = Quadrilateral {
        length: 40,
        width: 50,
    };

    let sq = Quadrilateral::create_square(10);

    println!("area of rectangle is: {}", rect.area());
    println!("Rectangle is: {:?}", rect);
    println!("Can rect hold rect2?: {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3?: {}", rect.can_hold(&rect3));
    println!("Here's a square for you: {:?}", sq);
}
