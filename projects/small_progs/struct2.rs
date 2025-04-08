
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let p = Person::new("John", "Smith");
    println!("person {} {}", p.first_name, p.last_name);
    println!("Fullname {}", p.full_name());
}