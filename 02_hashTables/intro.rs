#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    magic: bool,
    spell: String,
}

impl User {
    fn new(name: String, age: u32, magic: bool) -> Self {
        User {
            name,
            age,
            magic,
            spell: String::from(""), // Initialize the spell field
        }
    }

    fn scream(&self) {
        println!("Gaaaaaaggh!");
    }
}

fn main() {
    let mut user = User {
        name: String::from("Seth"),
        age: 54,
        magic: true,
        spell: String::from(""),
    };

    println!("{}", user.name); // Lookup O(1)
    user.spell = String::from("abra kadabra"); // Insert O(1)
                                               // println!("{:?}", user);
    println!("{:?}", user); // Lookup O(1)
}
