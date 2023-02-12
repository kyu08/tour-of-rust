fn main() {
    // x の型を推論
    let x = 13;
    println!("{}", x);

    let x: f64 = 3.14;
    println!("{}", x);

    let x;
    x = 0;
    println!("{}", x);
}
