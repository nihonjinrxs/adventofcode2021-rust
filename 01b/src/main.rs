use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type LinesResult = io::Result<io::Lines<io::BufReader<File>>>;

fn read_lines<P>(filename: P) -> LinesResult where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct IncreaseCounter {
    prev_prev:   Option<i32>,
    prev:        Option<i32>,
    prev_window: Option<i32>,
    count:       u32,
}

impl IncreaseCounter {
    fn new() -> Self {
        Self {
            prev_prev:   None,
            prev:        None,
            prev_window: None,
            count:       0u32,
        }
    }

    fn maybe_increment(&self, next_line: &str) -> Self {
        let next_val = next_line.clone().parse::<i32>().unwrap();
        match self.prev_prev {
            None => Self {
                prev_prev:   Some(next_val),
                prev:        None,
                prev_window: None,
                count:       self.count,
            },
            Some(prev_prev) => match self.prev {
                None => Self {
                    prev_prev:   Some(prev_prev),
                    prev:        Some(next_val),
                    prev_window: None,
                    count:       self.count,
                },
                Some(prev) => match self.prev_window {
                    None => Self {
                        prev_prev:   Some(prev_prev),
                        prev:        Some(prev),
                        prev_window: Some(
                            prev_prev + prev + next_val
                        ),
                        count:       self.count,
                    },
                    Some(prev_window) => {
                        let next_window: i32 = prev_prev + prev + next_val;
                        match next_window {
                            x if x >  prev_window => Self {
                                prev_prev:   Some(prev),
                                prev:        Some(next_val),
                                prev_window: Some(next_window),
                                count:       self.count + 1u32,
                            },
                            x if x <= prev_window => Self {
                                prev_prev:   Some(prev),
                                prev:        Some(next_val),
                                prev_window: Some(next_window),
                                count:       self.count,
                            },
                            _ => {
                                println!("Unparseable next_line! Skipping value! {}", &next_line);
                                Self {
                                    prev_prev:   Some(prev),
                                    prev:        Some(next_val),
                                    prev_window: Some(next_window),
                                    count:       self.count,
                                }
                            },            
                        }
                    },
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
    println!("Day 1, Part B. Counted {} increases using 3 day windows.", &counter.count)
}
