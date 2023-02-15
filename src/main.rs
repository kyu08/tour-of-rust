fn main() -> Result<(), String> {
    let age = 19;
    let v = int_to_result(age)?;
    println!("{}", v);
    Ok(())
}

fn int_to_result(age: i32) -> Result<i32, String> {
    if age > 18 {
        return Ok(12);
    }
    Err(String::from("you are 未成年！"))
}
