use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use common;
use common::AocResultType;
use common::AocResult;

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/01/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 1,
        phase_1: None,
        phase_2: None,
    };

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let mut floor = 0;
        let mut basement_idx: Option<usize> = None;
        for (i, c) in line.chars().enumerate() {
            floor = floor +
                    match c {
                        '(' => 1,
                        ')' => -1,
                        _ => panic!("Bad char: {} ", c),
                    };
            if floor == -1 && basement_idx == None {
                basement_idx = Some(i + 1);
            }
        }
        res.phase_1 = Some(AocResultType::Isize(floor));
        if let Some(x) = basement_idx {
            res.phase_2 = Some(AocResultType::Usize(x));
        }
    }
    res
}
