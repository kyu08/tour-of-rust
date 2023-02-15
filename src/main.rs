struct User {
    name: String,
    age: Option<u8>,
}

fn main() {
    let u = User {
        name: String::from("a"),
        age: Some(1),
    };

    println!("{},{}", u.name, age_to_string(u.age))
}

fn age_to_string(age: Option<u8>) -> String {
    match age {
        Some(a) => a.to_string(),
        None => String::from("not registered"),
    }
}
