use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let lines = match read_lines("./input") {
        Ok(lines) => lines,
        Err(err) => panic!("Could not read lines: {}", err),
    };

    let mut sum: u64 = 0;

    let mut line_count = 0;

    let mut backpack_1: HashSet<char> = HashSet::new();
    let mut backpack_2: HashSet<char> = HashSet::new();
    let mut backpack_3: HashSet<char> = HashSet::new();

    for line in lines {
        if let Ok(backpack) = line {
            match line_count {
                0 => backpack_1 = backpack.chars().collect(),
                1 => backpack_2 = backpack.chars().collect(),
                2 => {
                    backpack_3 = backpack.chars().collect();
                    line_count = 0;

                    let inter_1_2: HashSet<char> = backpack_1.intersection(&backpack_2).map(|&x| x).collect();
                    let full_inter = inter_1_2.intersection(&backpack_3);
                    let inter_sum: u64 = full_inter.map(|&x| calc_item_prio(x)).sum();
                    sum += inter_sum;
                    backpack_1.clear();
                    backpack_2.clear();
                    backpack_3.clear();
                    continue
                },
                i32::MIN..=-1_i32 | 3_i32..=i32::MAX => todo!(),
            }

            line_count += 1;
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

const LOWER_OFFSET: u64 = 96;
const UPPER_OFFSET: u64 = 38;

fn calc_item_prio(item: char) -> u64 {
    let dec = item as u64;
    if item.is_lowercase() {
        dec - LOWER_OFFSET
    } else {
        dec - UPPER_OFFSET
    }
}

fn insert_backpack_to_set(backpack: &str) -> HashSet<u64> {
    let mut result = HashSet::new();
    for char in backpack.chars() {
        result.insert(calc_item_prio(char));
    }
    result
}

fn inplace_intersection<T>(a: &mut HashSet<T>, b: &mut HashSet<T>) -> HashSet<T>
    where
        T: Hash,
        T: Eq,
{
    let c: HashSet<T> = a.iter().filter_map(|v| b.take(v)).collect();

    a.retain(|v| !c.contains(&v));

    c
}