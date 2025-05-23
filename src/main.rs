use rand::Rng;
use std::fs::File;
use std::io::{self, Write};
mod names;
use crate::names::{FEMALE_FIRST_NAMES, LAST_NAMES, MALE_FIRST_NAMES};

fn main() -> std::io::Result<()> {
    // Prompt user for number of names to generate
    print!("How many French names would you like to generate? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name_count: usize = match input.trim().parse() {
        Ok(num) => {
            if num > 0 {
                num
            } else {
                println!("Please enter a positive number. Using default: 1000");
                1_000
            }
        }
        Err(_) => {
            println!("No input. Using default of 1_000");
            1_000
        }
    };

    let mut first_names = MALE_FIRST_NAMES.to_vec();
    first_names.extend(FEMALE_FIRST_NAMES);

    let last_names = LAST_NAMES;

    let mut rng = rand::thread_rng();
    let mut file = File::create("french_names.csv")?;

    // Write CSV header
    writeln!(file, "firstname,lastname")?;

    // Generate random name combinations
    for _ in 0..name_count {
        let first_name = first_names[rng.gen_range(0..first_names.len())];
        let last_name = last_names[rng.gen_range(0..last_names.len())];
        writeln!(file, "{},{}", first_name, last_name)?;
    }

    println!("Successfully generated {name_count} French names in 'french_names.csv'");
    Ok(())
}
