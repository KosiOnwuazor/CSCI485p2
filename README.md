use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn linear_motion(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<String> {
    let mut positions = Vec::new();
    // Calculate the number of steps for each axis
    let steps = f64::max(f64::abs(end.0 - start.0), f64::max(f64::abs(end.1 - start.1), f64::abs(end.2 - start.2)));
    // Generate positions in one-unit increments
    for i in 0..=steps as usize {
        let x = (start.0 + (end.0 - start.0) * i as f64 / steps).round();
        let y = (start.1 + (end.1 - start.1) * i as f64 / steps).round();
        let z = (start.2 + (end.2 - start.2) * i as f64 / steps).round();
        positions.push(format!("LIN X{:.2} Y{:.2} Z{:.2}", x, y, z));
    }
    positions
}

fn execute_command(command: &str) {
    // Implement the logic to execute rotational motion commands
    println!("Executing command: {}", command);
}

fn main() {
    // Read commands from a file
    if let Ok(lines) = read_lines("commands.txt") {
        // Process each line in the file
        for line in lines {
            if let Ok(cmd) = line {
                // Parse the command
                let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
                if parts.len() >= 4 {
                    let action = parts[0];
                    let start = (
                        parts[1].parse::<f64>().unwrap_or(0.0),
                        parts[2].parse::<f64>().unwrap_or(0.0),
                        parts[3].parse::<f64>().unwrap_or(0.0),
                    );
                    let end = (
                        parts[4].parse::<f64>().unwrap_or(0.0),
                        parts[5].parse::<f64>().unwrap_or(0.0),
                        parts[6].parse::<f64>().unwrap_or(0.0),
                    );
                    // Execute linear motion or rotational motion based on the action
                    if action == "LIN" {
                        let positions = linear_motion(start, end);
                        for pos in positions {
                            println!("{}", pos);
                        }
                    } else {
                        let command = parts[0..].join(" ");
                        execute_command(&command);
                    }
                }
            }
        }
    }
}

// Function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
