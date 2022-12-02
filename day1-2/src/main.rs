use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut heap = BinaryHeap::with_capacity(4);

    if let Ok(lines) = read_lines("./input") {
        let mut current_elf = 0;
        for line in lines {
            if let Ok(calories) = line {
                if calories == "" {
                    heap.push(Reverse(current_elf));
                    if heap.len() > 3 {
                        heap.pop();
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

    let mut result: i32 = 0;
    for elf in heap.iter() {
        result += elf.0;
    }
    println!("{}", result)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
