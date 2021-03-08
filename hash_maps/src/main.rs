use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 20);

    let k = scores.get("blue");

    match k {
        None => println!("Nope"),
        Some(val) => println!("Do {} times", val), 
    }

    let k = scores.get("green");

     match k {
        None => println!("Nope"),
        Some(val) => println!("Do {} times", val), 
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
