fn main() {
    let s = String::from("Hello world!");
    let first_word = first_word(&s);

    println!("first word: {}", first_word);
}

fn first_word(s: &String) -> &str {
    let s_b = s.as_bytes();
    for (i, &char) in s_b.iter().enumerate() {
        if char == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
