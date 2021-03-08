#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(5.5),
        SpreadsheetCell::Text(String::from("It's a beautiful world!")),
    ];

    println!("{:#?}", row);
}