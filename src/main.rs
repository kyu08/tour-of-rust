fn main() {
    match int_to_result(100) {
        Ok(a) => println!("{}円つかっていいよ", a),
        Err(e) => println!("{}", e),
    }
}

fn int_to_result(age: i32) -> Result<i32, String> {
    let number = 18;
    if age > number {
        return Ok(12);
    }
    Err(String::from("you are 未成年！"))
}
