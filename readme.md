# Pokemon Data Reader

## Overview

A simple Rust application to read and display data from a CSV file containing Pokémon information.

## Features

- Reads data from a specified CSV file.
- Parses each record into a structured format.
- Displays detailed information about each Pokémon.

## Usage

1. Run the program.
2. Enter the path to your CSV file when prompted, or press enter to use the default path (`pokemon_data.csv`).

## Record Structure

Each record in the CSV file should adhere to the following structure:

- `name`: The name of the Pokémon.
- `pokemon_type`: The Pokémon's type.
- `hp`: Health Points.
- `attack`: Attack rating.
- `defense`: Defense rating.

## Dependencies

- `csv`: For parsing CSV files.
- `serde`: For deserializing data into Rust structures.

## Error Handling

Errors in reading the CSV file or processing records are reported to the user.

---

Feel free to contribute or suggest improvements!
