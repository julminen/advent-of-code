use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::collections::BTreeMap;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn new(spec: &str) -> Reindeer {
        let tokens: Vec<&str> = spec.split_whitespace().collect();
        let name = tokens[0].to_string();
        let speed = usize::from_str(tokens[3]).unwrap();
        let fly_time = usize::from_str(tokens[6]).unwrap();
        let rest_time = usize::from_str(tokens[13]).unwrap();

        Reindeer {
            name: name,
            speed: speed,
            fly_time: fly_time,
            rest_time: rest_time,
        }
    }

    fn distance(&self, time: usize) -> usize {
        let mut distance = 0;
        let cycle_time = self.fly_time + self.rest_time;
        let full_cycles: usize = time / cycle_time;
        let partial_time = time % cycle_time;
        let partial_fly_time = if partial_time < self.fly_time {
            partial_time
        } else {
            self.fly_time
        };


        distance += full_cycles * (self.fly_time * self.speed);
        distance += partial_fly_time * self.speed;

        distance
    }
}

fn load_file(file_name: &str) -> Vec<Reindeer> {
    let mut reindeer: Vec<Reindeer> = Vec::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        reindeer.push(Reindeer::new(&line));
    }

    reindeer
}

fn v2_points(reindeer: &Vec<Reindeer>, fly_time: usize) -> usize {
    let mut points: BTreeMap<String, usize> = BTreeMap::new();
    for r in reindeer {
        points.insert(r.name.clone(), 0);
    }

    for t in 1..fly_time + 1 {
        let mut max_dist = 0;
        for r in reindeer {
            let dist = r.distance(t);
            if dist > max_dist {
                max_dist = dist;
            }
        }
        for r in reindeer {
            let dist = r.distance(t);
            if dist == max_dist {
                let mut value = points.get_mut(&r.name).unwrap();
                *value = *value + 1;
            }
        }
    }

    let mut max_points = 0;
    for (_, value) in points {
        //println!("{}: {}", key, value);
        if value > max_points {
            max_points = value;
        }
    }
    max_points
}

pub fn solve(file_name: Option<&str>, fly_time: Option<usize>) -> AocResult {
    let mut sim_time = 2503;
    let mut full_name = String::from("input/14/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }
    match fly_time {
        Some(time) => sim_time = time,
        None => {
            let time = common::get_input("Fly time", &sim_time.to_string()).parse();
            match time {
                Ok(v) => sim_time = v,
                Err(_) => println!("Cannot parse '{}', using default", sim_time),
            }
        }
    }

    let reindeer = load_file(&full_name);
    let mut max_distance = 0;
    for r in &reindeer {
        //println!("{:?}, ft {}", r, r.distance(sim_time));
        let distance = r.distance(sim_time);
        if r.distance(sim_time) > max_distance {
            max_distance = distance;
        }
    }

    let v2 = v2_points(&reindeer, sim_time);

    let res: AocResult = AocResult {
        day: 14,
        phase_1: Some(AocResultType::Usize(max_distance)),
        phase_2: Some(AocResultType::Usize(v2)),
    };

    res
}
