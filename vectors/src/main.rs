fn main() {
    let mut v: Vec<i32> = Vec::new();

    let mut v2 = vec![1, 2, 3, 4, 5];
    v.push(3);
    v.push(4);
    v.pop();

    println!("{:?}", v);
    println!("{:?}", v2);

    let third: &i32 = &v2[2];
    println!("third value is {}", third);

    match v2.get(2) {
        Some(third) => println!("third: {}", third),
        None => println!("Nope. nothing found!"),
    }

    // for e in v2 {
    //     println!("{}", e);
    // }

    for e in &mut v2 {
        *e += 10;
    }

    for e in v2 {
        println!("{}", e);
    }

    println!("Hello, world!");
}
