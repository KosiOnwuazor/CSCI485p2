use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
/// Reads lines from a file and returns an iterator over the lines.
/// 
/// # Arguments
/// 
/// * `filename` - A path to the file to read.
/// 
/// # Returns
/// 
/// An iterator over the lines of the file, wrapped in an `io::Result`.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Computes positions for linear motion between two points.
/// 
/// # Arguments
/// 
/// * `start` - The starting point of the motion (x, y, z).
/// * `end` - The ending point of the motion (x, y, z).
/// 
/// # Returns
/// 
/// A vector containing the computed positions along the linear path.
fn linear_motion(start: (f64, f64, f64), end: (f64, f64, f64)) -> Vec<String> {
    let mut positions = Vec::new();
    let steps = f64::max(
        f64::abs(end.0 - start.0),
        f64::max(f64::abs(end.1 - start.1), f64::abs(end.2 - start.2)),
    );
    for i in 0..=steps as usize {
        let x = start.0 + (end.0 - start.0) * i as f64 / steps;
        let y = start.1 + (end.1 - start.1) * i as f64 / steps;
        let z = start.2 + (end.2 - start.2) * i as f64 / steps;
        positions.push(format!("{:.2}, {:.2}, {:.2}", x, y, z));
    }
    positions
}
/// Computes positions for rotational motion around a center point.
/// 
/// # Arguments
/// 
/// * `position` - The center point of the rotation (x, y).
/// * `radius` - The radius of the circle.
/// * `_direction` - The direction of rotation (not used in this implementation).
/// * `_stop` - The stop point of the rotation (not used in this implementation).
/// 
/// # Returns
/// 
/// A vector containing the computed positions along the circular path.
fn rotational_motion(
    position: (f64, f64),
    radius: f64,
    _direction: &str,
    _stop: (f64, f64),
) -> Vec<String> {
    let mut positions = Vec::new();
    let (center_x, center_y) = position;
    let mut angle: f64 = 0.0;
    let step = 5.0_f64.to_radians();

    while angle <= 360.0 {
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        positions.push(format!("{:.2}, {:.2}", x, y));
        angle += step;
    }

    positions
}

fn main() {
    // Read commands from a file
    if let Ok(lines) = read_lines("commands.txt") {
        // Print the contents of the file before processing
        for line in lines {
            println!("{}", line.unwrap());
        }

        // Process each line in the file
        if let Ok(lines) = read_lines("commands.txt") {
            for line in lines {
                if let Ok(cmd) = line {
                    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
                    println!("Parsed command parts: {:?}", parts); // Debugging print
                    if parts.len() >= 7 {
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
                        if action == "LIN" {
                            println!("Executing linear motion"); // Debugging print
                            let positions = linear_motion(start, end);
                            for (index, pos) in positions.iter().enumerate() {
                                println!("{}. {}", index + 1, pos);
                            }
                        } else if action == "ROT" {
                            println!("Executing rotational motion"); // Debugging print
                            let position = (
                                parts[1].parse::<f64>().unwrap_or(0.0),
                                parts[2].parse::<f64>().unwrap_or(0.0),
                            );
                            let radius = parts[3].parse::<f64>().unwrap_or(0.0);
                            let direction = parts[4];
                            let stop = (
                                parts[5].parse::<f64>().unwrap_or(0.0),
                                parts[6].parse::<f64>().unwrap_or(0.0),
                            );
                            let positions =
                                rotational_motion(position, radius, direction, stop);
                            for (index, pos) in positions.iter().enumerate() {
                                println!("{}. {}", index + 1, pos);
                            }
                        } else {
                            // Handle other actions
                        }
                    }
                }
            }
        } else {
            println!("Error reading commands from file.");
        }
    }
}
