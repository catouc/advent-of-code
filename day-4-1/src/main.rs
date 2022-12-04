use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = match read_lines("./input") {
        Ok(lines) => lines,
        Err(err) => panic!("Could not read lines: {}", err),
    };

    let mut contained_count = 0;
    for line in lines {
        if let Ok(sections) = line {
            let split: Vec<&str> = sections.split(",").collect();
            let contained = is_contained(split[0], split[1]);
            if contained {
                contained_count += 1;
            }
        }
    }

    println!("{}", contained_count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// section 1 lower x, upper y
// section 2 lower a, upper b
// section 1 is fully in section 2 if x >= a && y <= b
fn is_contained(a: &str, b: &str) -> bool {
    let (a_lower, a_upper) = split_section(a);
    let (b_lower, b_upper) = split_section(b);
    (a_lower >= b_lower && a_upper <= b_upper) || (b_lower >= a_lower && b_upper <= a_upper)
}

fn split_section(section: &str) -> (i32, i32) {
    let split: Vec<i32> = section.split("-").map(|x|x.parse().unwrap()).collect();
    (split[0], split[1])
}