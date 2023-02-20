fn main() {
    let chars = "hi".chars().collect::<Vec<char>>();
    println!("{}", chars.len());
    println!("{}", chars[3] as u32);
}
