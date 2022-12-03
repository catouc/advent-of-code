use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    let lines = match read_lines("./input") {
        Ok(lines) => lines,
        Err(err) => panic!("Could not read lines: {}", err),
    };

    let lower_offset = 96;
    let upper_offset = 38;
    let mut sum: u64 = 0;

    for line in lines {
        // 12 items per compartment
        if let Ok(backpack) = line {
            let compartment_1 = &backpack[..backpack.len()/2];
            let compartment_2 = &backpack[backpack.len()/2..];

            let mut duplicates = HashSet::new();
            for c1_char in compartment_1.chars() {
                for c2_char in compartment_2.chars() {
                    if c1_char == c2_char {
                        duplicates.insert(c1_char);
                    }
                }
            }

            for dupe in duplicates.iter() {
                let dec = *dupe as u64;
                if dupe.is_lowercase() {
                    sum += dec - lower_offset;
                } else {
                    sum += dec - upper_offset;
                }
            }
        }
    }

    println!("{}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}