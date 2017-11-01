use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

#[derive(Debug)]
struct Disc {
    id: usize,
    position_count: usize,
    start_position: usize,
}

impl Disc {
    pub fn new(id: usize, position_count: usize, start_position: usize) -> Disc {
        Disc {
            id: id,
            position_count: position_count,
            start_position: start_position,
        }
    }
    pub fn parse(text_repr: String) -> Disc {
        let tokens: Vec<&str> = text_repr.split(|c| c == ' ' || c == '.' || c == '#' || c == '=').collect();
        
        Disc::new(tokens[2].parse().unwrap(), tokens[4].parse().unwrap(), tokens[13].parse().unwrap())
    }

    pub fn is_droppable_position(&self, time: usize) -> bool {
        (self.id + self.start_position + time) % self.position_count == 0
    }
}

fn parse_file(file_name: String) -> Vec<Disc> {
    let mut discs: Vec<Disc> = Vec::new();

    println!("Reading {}", file_name);
    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let d = Disc::parse(line);
        println!("Disc: {:?}", d);
        discs.push(d);
    }
    discs
}

fn main() {
    println!("AOC 15");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    let discs = parse_file(file_name);

    // Simulate
    for time in 0..usize::max_value() {
        // println!("Time: {}", time);
        let mut droppable = true;
        for disc in &discs {
            droppable = droppable && disc.is_droppable_position(time);
            if ! droppable {
                break;
            }
        }
        if droppable {
            println!("\n\nFirst droppable at {}", time);
            break;
        }
    }

}
