fn main() {
    let (x, y) = tuple_func(1);
    println!("x:{}, y: {}", x, y);
}

fn tuple_func(x: i32) -> (i32, i32) {
    (x, x + 1)
}
