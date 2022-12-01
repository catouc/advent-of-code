use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut highest_elf = 0;
    if let Ok(lines) = read_lines("./input") {
        let mut current_elf = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    if current_elf > highest_elf {
                        highest_elf = current_elf;
                    }
                    current_elf = 0;
                    continue
                }

                match calories.parse::<i32>() {
                    Ok(cal) => current_elf += cal,
                    Err(err) => eprintln!("Fuck: {}", err)
                }
            }
        }
    }
    println!("{}", highest_elf)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
