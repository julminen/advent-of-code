use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Range {
    start: u32,
    end: u32,
}

impl Ord for Range {
    fn cmp(&self, other: &Range) -> Ordering {
        if self.start < other.start {
            Ordering::Less
        } else if self.start > other.start {
            Ordering::Greater
        } else {
            if self.end < other.end {
                Ordering::Less
            } else if self.end > other.end {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Range) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    println!("AOC 20");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "example".to_string()
    };

    println!("Reading {}", file_name);

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let mut ranges: Vec<Range> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        let parts: Vec<&str> = line.split('-').collect();
        ranges.push(
            Range {
                start: parts[0].parse().unwrap(),
                end: parts[1].parse().unwrap(),
            }
            );
    }

    ranges.sort();
    let mut max: u32 = ranges[0].end;
    let mut lowest: u32 = 0;
    let mut count: u32 = 0;
    for r in ranges {
        println!("{} - {}", r.start, r.end);
        if r.end == u32::max_value() {
            println!("At the end (end = {})", r.end);
        }
        
        if max < u32::max_value() && r.start > max + 1 {
            if lowest == 0 {
                lowest = max + 1;
            }
            let add = r.start - max - 1;
            count = count + add;
            println!("!!! range start {} after max {} : count {}",
                     r.start, max, add);
        }
        if r.end > max {
            max = r.end;
        }
    }
    //count = count + range_max - max;
    println!("Lowest: {}, count: {}", lowest, count);
}
