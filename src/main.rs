#![allow(dead_code)] // この行でコンパイラのwaringsメッセージを止める

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct SeaCreature {
    species: Species,
    name: String,
    age: i32,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("ferris"),
        age: 3,
    };

    match ferris.species {
        Species::Crab => println!("crab"),
        Species::Octopus => println!("octopus"),
        Species::Fish => println!("fish"),
        Species::Clam => println!("clam"),
    }
}
