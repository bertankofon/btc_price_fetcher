
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_mode() {
    let filename = "btc_prices.txt";

    let file = File::open(filename);
    let reader = match file {
        Ok(file) => BufReader::new(file),
        Err(error) => {
            eprintln!("Error opening file '{}': {}", filename, error);
            return;
        }
    };

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                eprintln!("Error reading file '{}': {}", filename, error);
                return;
            }
        }
    }
}
