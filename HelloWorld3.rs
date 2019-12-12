struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn say_name(&self) {
        println!("I'm {}, {} years old.", self.name, self.age);
    }
}

fn main() {
    let person = Person {name: "armorik".to_string(), age: 83};
    person.say_name();
}
