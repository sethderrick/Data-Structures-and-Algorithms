struct User {
    name: String,
    age: u32,
    magic: bool,
}

impl User {
    fn scream(&self) {
        println!("Gaaaaaaggh!");
    }
}

fn main() {
    let mut user = User {
        name: String::from("Seth"),
        age: 54,
        magic: true,
    };

    println!("{}", user.name); // Lookup O(1)
    user.spell = String::from("abra kadabra"); // Insert O(1)
    println!("{:?}", user);
}
