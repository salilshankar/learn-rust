fn main() {
    let mut s = "iterable...";
    s = &s[0..4]; 
    println!("test slice, {}", s);

    let s2 = String::from("hello-world!");

    s = &s2[0..5];

    println!("{}", s);

    let s1 = String::from("big");
    let s2 = String::from("...unfurl");
    let s3 = s1 + &s2;

    println!("{}", s3);

    //slicing at &hello[0..1] won't work because 
    //a Hindi character consumes 3 bytes vs English 
    //where a character consumes 1. 
    let hello = "नमस्ते";
    let sx = &hello[0..3];

    println!("Hindi char (takes 3 bytes): {}", sx);

    for c in hello.chars() {
        println!("hindi char: {}", c);
    }

    let hello = "Exercise";
    let sx = &hello[0..1];

    println!("English char (takes 1 byte): {}", sx);
}
