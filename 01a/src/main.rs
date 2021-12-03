use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type LinesResult = io::Result<io::Lines<io::BufReader<File>>>;

fn read_lines<P>(filename: P) -> LinesResult where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct IncreaseCounter {
    prev_val: Option<i32>,
    count: u32,
}

impl IncreaseCounter {
    fn new() -> Self {
        Self { prev_val: None, count: 0u32 }
    }

    fn maybe_increment(&self, next_line: &str) -> Self {
        let next_val = next_line.clone().parse::<i32>().unwrap();
        match self.prev_val {
            None => Self { prev_val: Some(next_val), count: self.count },
            Some(val) => match next_val {
                x if x > val => Self { prev_val: Some(x), count: self.count + 1u32 },
                x if x <= val => Self { prev_val: Some(x), count: self.count },
                _ => {
                    println!("Unparseable next_line! Skipping value! {}", &next_line);
                    Self { prev_val: None, count: self.count }
                },
            },
        }
    }
}

fn main() {
    let mut counter = IncreaseCounter::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(value_string) = line {
                counter = counter.maybe_increment(&value_string)
            }
        }
    }
    println!("Day 1, Part A. Counted {} increases.", &counter.count)
}
