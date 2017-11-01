use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeSet;

use common;
use common::AocResultType;
use common::AocResult;

#[derive(Ord, Eq, PartialOrd, PartialEq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
    fn up(&mut self) {
        self.y += 1;
    }
    fn down(&mut self) {
        self.y -= 1;
    }
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/03/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 3,
        phase_1: None,
        phase_2: None,
    };

    // phase 1: one santa
    let mut set: BTreeSet<Point> = BTreeSet::new();
    let mut current = Point { x: 0, y: 0 };
    set.insert(current.clone());

    // phase 2: add robo santa
    let mut robo_set: BTreeSet<Point> = BTreeSet::new();
    let mut santa = current.clone();
    let mut robo = current.clone();
    let mut santa_turn: bool = true;
    robo_set.insert(santa.clone());

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        for c in line.chars() {
            match c {
                '^' => {
                    current.up();
                    match santa_turn {
                        true => santa.up(),
                        false => robo.up(),
                    }
                }
                '<' => {
                    current.left();
                    match santa_turn {
                        true => santa.left(),
                        false => robo.left(),
                    }
                }
                '>' => {
                    current.right();
                    match santa_turn {
                        true => santa.right(),
                        false => robo.right(),
                    }
                }
                'v' => {
                    current.down();
                    match santa_turn {
                        true => santa.down(),
                        false => robo.down(),
                    }
                }
                _ => panic!("Bad direction: {}", c),
            }
            set.insert(current.clone());
            let rs = match santa_turn {
                true => santa.clone(),
                false => robo.clone(),
            };
            robo_set.insert(rs);
            santa_turn = !santa_turn;
        }
    }
    res.phase_1 = Some(AocResultType::Usize(set.len()));
    res.phase_2 = Some(AocResultType::Usize(robo_set.len()));

    res
}
