fn main() {
    let beasts = vec![
        "Centaur", "Godzilla", "Mosura", "Minotaur", "Hydra", "Nessie",
    ];

    // Finding the index of "Godzilla"
    let index_of_godzilla = beasts.iter().position(|&item| item == "Godzilla");
    println!("Index of Godzilla: {:?}", index_of_godzilla); // Outputs: Some(1)

    // Rust's `find` method is used to get a reference to the first element that matches the condition
    let find_godzilla = beasts.iter().find(|&&item| item == "Godzilla");
    println!("Found Godzilla: {:?}", find_godzilla); // Outputs: Some("Godzilla")

    // Checking if "Godzilla" is included in the vector
    let includes_godzilla = beasts.contains(&"Godzilla");
    println!("Includes Godzilla: {}", includes_godzilla); // Outputs: true
}
