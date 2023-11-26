use std::{
    fmt::{self, Display},
    io,
};

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

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nType: {}\nHP: {}\nAttack: {}\nDefense: {}\n------------------",
            self.name, self.pokemon_type, self.hp, self.attack, self.defense
        )
    }
}

fn read_from_file(path: &str) -> Result<Vec<Record>, Error> {
    let mut reader = csv::Reader::from_path(path).map_err(|error| {
        eprintln!("Failed to read file: {}", error);
        error
    })?;

    let mut records = Vec::<Record>::new();

    for record in reader.deserialize::<Record>() {
        let result = record?;
        println!("{}", result);
        records.push(result);
    }

    Ok(records)
}

fn get_path_from_user(default: &str) -> String {
    let mut path = String::new();

    println!("Enter the path to the CSV file (default: {}):", default);
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");

    if path.trim().is_empty() {
        default.to_string()
    } else {
        path.trim().to_string()
    }
}

fn main() {
    let path = get_path_from_user(CSV_PATH);
    if let Err(e) = read_from_file(&path) {
        eprintln!("{}", e);
    }
}
