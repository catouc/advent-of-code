use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = match read_lines("./input") {
        Ok(lines) => lines,
        Err(err) => panic!("Could not read lines: {}", err),
    };

    let mut score = 0;
    for line in lines {
        if let Ok(game) = line {
            score += play(&game);
        }
    }

    println!("{}", score);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn play(game: &str) -> i32 {
    // A: Rock, B: Paper, C: Scissors
    // X: Rock, Y: Paper, Z: Scissors
    // Rock: 1, Paper: 2, Scissors: 3
    // Win: 6, Draw: 3, Loss: 0
    let win = 6;
    let draw = 3;
    let loss = 0;
    let rock = 1;
    let paper = 2;
    let scissors = 3;

    match game {
        "A X" => draw + rock,
        "A Y" => win + paper,
        "A Z" => loss + scissors,
        "B X" => loss + rock,
        "B Y" => draw + paper,
        "B Z" => win + scissors,
        "C X" => win + rock,
        "C Y" => loss + paper,
        "C Z" => draw + scissors,
        &_ => 0,
    }
}