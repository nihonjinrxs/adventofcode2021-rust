use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type LinesResult = io::Result<io::Lines<io::BufReader<File>>>;

fn read_lines<P>(filename: P) -> LinesResult where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Submarine {
    position: u32,
    depth:    u32,
}

enum SubmarineCommand {
    Forward,
    Down,
    Up,
}

impl Submarine {
    fn new() -> Self {
        Self { position: 0u32, depth: 0u32 }
    }

    fn forward(&self, distance: u32) -> Self {
        Self {
            position: self.position + distance,
            depth:    self.depth,
        }
    }

    fn down(&self, distance: u32) -> Self {
        Self {
            position: self.position,
            depth:    self.depth + distance,
        }
    }

    fn up(&self, distance: u32) -> Self {
        Self {
            position: self.position,
            depth:    self.depth - distance,
        }
    }

    fn parse(&self, command_string: &str) -> Option<(SubmarineCommand, u32)> {
        let mut parts = command_string.trim().split_whitespace();
        let command: Option<&str> = parts.next();
        let distance: u32 = parts.next().clone().unwrap().parse().unwrap();
        match command {
            Some("forward") => Some((SubmarineCommand::Forward, distance)),
            Some("down") => Some((SubmarineCommand::Down, distance)),
            Some("up") => Some((SubmarineCommand::Up, distance)),
            _ => None 
        }
    }

    fn evaluate(&self, command: SubmarineCommand, distance: u32) -> Self {
        match command {
            SubmarineCommand::Forward => self.forward(distance),
            SubmarineCommand::Down    => self.down(distance),
            SubmarineCommand::Up      => self.up(distance),
        }
    }

    fn value(&self) -> u32 {
        self.position * self.depth
    }
}

fn main() {
    let mut submarine = Submarine::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value_string) = line {
                match submarine.parse(&value_string) {
                    Some(tup) => {
                        let command: SubmarineCommand = tup.0;
                        let distance: u32 = tup.1;
                        submarine = submarine.evaluate(command, distance)
                    },
                    _ => {}
                }
            }
        }
    }
    println!(
        "Day 2, Part A. Submarine is at ({}p, {}d), with value {}.",
        &submarine.position,
        &submarine.depth,
        &submarine.value(),
    )
}
