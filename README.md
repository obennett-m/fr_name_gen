# French Name Generator

A simple command-line tool written in Rust that generates random French names and saves them to a CSV file. This tool combines authentic French first names and last names to create realistic name combinations suitable for testing, data generation, or creative writing.

## Features

- Generates random combinations of French first and last names
- Customizable number of names to generate
- Saves output directly to a CSV file

## Installation

### Prerequisites

- Rust and Cargo (installation instructions at [rust-lang.org](https://www.rust-lang.org/tools/install))

### Building from Source

1. Clone this repository
2. Build the project:

```bash
cd fr_name_gen
cargo build --release
```

The compiled binary will be located at `./target/release/fr_name_gen`

## Usage

Run the program:

```bash
cargo run
```

Or use the binary directly after building:

```bash
./target/release/fr_name_gen
```

The program will prompt you to enter the number of names you want to generate:

```
How many French names would you like to generate?
```

Enter a positive number. If you don't enter anything or enter an invalid number, it will default to 1000.

The generated names will be saved to a file named `french_names.csv` in the current directory.

### Example

```
$ ./fr_name_gen
How many French names would you like to generate? 100
Successfully generated 100 French names in 'french_names.csv'
```

## Output Format

Example:

```
firstname,lastname
Adèle,Vavasseur
Jeanne,Thierry
Georges,Garnier
```

### Note on Duplicate Combinations

The current implementation uses random selection which can produce duplicate combinations.

- Maximum unique combinations possible: 79,488 (276 first names × 288 last names)
- When generating small batches (< 1,000 names) expect mostly unique combinations
- When generating larger batches (> 10,000 names) expect increasing number of duplicates
- When generating more than 79,488 names: duplicates inevitable


For applications requiring only unique name combinations, consider:
1. Post-processing the CSV to remove duplicates, e.g.:
   ```bash
   sort -u -t, -k1,2 french_names.csv > unique_french_names.csv
   ```

## Acknowledgements

The French names used in this project are based on [french-name-generator](https://github.com/yunkii/french-name-generator) by yunkii.

## Contributing

Contributions are welcome! Feel free to submit pull requests to add more French names or improve functionality.
