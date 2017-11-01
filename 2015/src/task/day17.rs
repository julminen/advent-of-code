use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Container {
    size: usize,
}

impl Container {
    fn new(spec: &str) -> Container {
        Container { size: usize::from_str(spec).unwrap() }
    }
}

fn get_subset(n: usize, containers: &Vec<Container>) -> Vec<Container> {
    let mut subset: Vec<Container> = Vec::with_capacity(containers.len());
    let mask: usize = 0x01;
    let mut n = n;
    for i in 0..containers.len() {
        if n & mask == 0x01 {
            subset.push(containers[i].clone());
        }
        n = n >> 1;
    }

    subset
}

fn sum_containers(containers: &Vec<Container>) -> usize {
    containers.iter().fold(0, |acc, x| acc + x.size)
}

fn get_matching_subsets(containers: &Vec<Container>, capacity: usize) -> (usize, usize, usize) {
    let mut matches = 0;
    let mut min_containers = usize::max_value();
    let mut min_container_count = 0;
    let combinations = (2 as usize).pow(containers.len() as u32);
    for combo in 0..combinations {
        let subset = get_subset(combo, &containers);
        let sum = sum_containers(&subset);
        // println!("Sum of {:?} is {}", subset, sum);
        if sum == capacity {
            //println!("Match {}: {:?}", combo, subset);
            matches += 1;
            if subset.len() < min_containers {
                min_container_count = 1;
                min_containers = subset.len();
            } else if subset.len() == min_containers {
                min_container_count += 1;
            }
        }
    }

    (matches, min_containers, min_container_count)
}

fn load_file(file_name: &str) -> Vec<Container> {
    let mut res: Vec<Container> = Vec::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        res.push(Container::new(&line));
    }

    res
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let freezer_space = 150;
    let mut full_name = String::from("input/17/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let containers = load_file(&full_name);

    let (matches, containers, count) = get_matching_subsets(&containers, freezer_space);
    println!("{} matches, need at least {} containers, {} options to choose from",
             matches,
             containers,
             count);

    let res: AocResult = AocResult {
        day: 17,
        phase_1: Some(AocResultType::Usize(matches)),
        phase_2: Some(AocResultType::Usize(count)),
    };

    res
}
