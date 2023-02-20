fn main() {
    let str = "hi🦀";
    match str.find('🦀') {
        Some(v) => println!("Got -> {}", v),
        None => println!("There is not any crabs..."),
    };
}
