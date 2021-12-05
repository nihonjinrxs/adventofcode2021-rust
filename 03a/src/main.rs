use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod bit_count;
mod bit_number;
mod power_consumption;

mod prelude {
    pub use crate::bit_count::*;
    pub use crate::bit_number::*;
    pub use crate::power_consumption::*;
}
use prelude::*;

type LinesResult = io::Result<io::Lines<io::BufReader<File>>>;

fn read_lines<P>(filename: P) -> LinesResult where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        let measurements: Vec<Vec<u32>> = lines
            .map(|line| if let Ok(l) = line {
                l.chars().map(|c| c.to_digit(2u32).unwrap()).collect()
            } else {
                vec![] as Vec<u32>
            })
            .collect();
        let power = PowerConsumption::generate_from_measurements(&measurements);
        println!(
            "Day 3, Part A. {:?} totaling {}.",
            &power,
            &power.value(),
        )
    }
}

#[cfg(test)]
#[test]
fn example_given_works() {
    if let Ok(lines) = read_lines("./test") {
        let measurements: Vec<Vec<u32>> = lines
            .map(|line| if let Ok(l) = line {
                l.chars().map(|c| c.to_digit(2u32).unwrap()).collect()
            } else {
                vec![] as Vec<u32>
            })
            .collect();
        let power = PowerConsumption::generate_from_measurements(&measurements);
        assert_eq!(power.gamma_rate, 22u32);
        assert_eq!(power.epsilon_rate, 9u32);
        assert_eq!(power.value(), 198u32);
    }
}
