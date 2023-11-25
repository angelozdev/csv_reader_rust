use std::io;

use csv::Error;
use serde::Deserialize;

const CSV_PATH: &str = "pokemon_data.csv";

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    pokemon_type: String,
    hp: u8,
    attack: u8,
    defense: u8,
}

fn read_from_file(path: &str) -> Result<(), Error> {
    let mut reader = csv::Reader::from_path(path)?;

    for record in reader.deserialize::<Record>() {
        let result = record?;
        println!("Name: {}", result.name);
        println!("Type: {}", result.pokemon_type);
        println!("HP: {}", result.hp);
        println!("Attack: {}", result.attack);
        println!("Defense: {}", result.defense);
        println!("------------------");
    }

    Ok(())
}

fn get_path_from_user(default: &str) -> String {
    let mut path = String::new();

    println!("Enter the path to the CSV file (default: {}):", default);
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    if path.trim() == "" {
        return default.to_string();
    }

    path.trim().to_string()
}

fn main() {
    let path = get_path_from_user(CSV_PATH);
    if let Err(e) = read_from_file(&path) {
        eprintln!("{}", e);
    }
}
