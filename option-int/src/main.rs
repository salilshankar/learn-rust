fn main() {
    let five = Some(5);
    let six = check_five(five);
    if let Some(v) = six {
        println!("{}", v);
    }
}

fn check_five(i: Option<i32>) -> Option<i32> {
    match i {
        Some(val) => Some(val + 1),
        None => None,
    }
}
