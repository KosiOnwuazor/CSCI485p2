use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to read file and parse text
fn read_file_and_parse(filename: &str) -> Vec<Vec<f64>> {
    let mut data = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                let mut row = Vec::new();
                for word in line.split_whitespace() {
                    if let Ok(value) = word.parse::<f64>() {
                        row.push(value);
                    }
                }
                data.push(row);
            }
        }
    }

    data
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_data = read_file_and_parse("input.txt");

    // Print the parsed data for verification
    for row in &input_data {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}
