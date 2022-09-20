pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}

pub fn main() -> () {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);

    println!("Bob's name: {:#?}", bob.name);

    println!("Bob's age: {:#?}", bob.age);
    bob.set_age(33);
    println!("Bob's new age: {:#?}", bob.age);

    println!("Bob's weight: {:#?}", bob.weight);
    bob.set_weight(100.0);
    println!("Bob's weight: {:#?}", bob.weight);
}
