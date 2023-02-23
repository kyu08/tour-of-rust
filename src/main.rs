struct User {
    name: String,
}

impl User {
    fn get_name(&self) -> &str {
        &self.name
    }
}

trait Runner {
    fn run(&self);
    fn say(&self);
}

impl Runner for User {
    fn run(&self) {
        println!("{} is running", self.get_name())
    }
    fn say(&self) {}
}

fn main() {
    let u = User {
        name: String::from("john"),
    };
    u.run();
}
