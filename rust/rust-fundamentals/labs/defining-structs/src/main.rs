#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: Option<String>, // optional field
    phone: Option<String>, // optional field
}

impl Person {
    fn new(first_name: String, last_name: String, age: u8) -> Self {
        Self {
            first_name,
            last_name,
            age,
            email: None,
            phone: None,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut p = Person::new(
        "John".to_string(), 
        "Doe".to_string(), 
        25
    );

    p.email = Some("john.doe@example.com".to_string());
    p.phone = Some("4158675309".to_string());

    println!("{:#?}", p);  // "{:#?}" does "pretty print", unlike "{:?}"

    let fullName = p.full_name();
    println!("Full name: {}", fullName);
}