use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

use common;
use common::AocResultType;
use common::AocResult;

fn count_memory(line: &str) -> usize {
    let mut len = 0;
    let chars: Vec<char> = line.chars().collect();
    if chars[0] != '"' || chars[0] != chars[chars.len() - 1] {
        println!("Bad line: {}", line);
        return 0;
    }
    let mut skip = 0;
    for l in 1..chars.len() - 1 {
        if skip > 0 {
            skip -= 1;
        } else {
            len += match chars[l] {
                '\\' => {
                    match chars[l + 1] {
                        '\\' | '"' => {
                            skip = 1;
                            1
                        }
                        'x' => {
                            skip = 3;
                            1
                        }
                        _ => panic!("Bad encoding in line {}", line),
                    }
                }
                _ => 1,
            }
        }
    }

    len
}

fn encode(line: &str) -> String {
    let mut encoded: Vec<char> = Vec::with_capacity(line.len() * 2);
    let chars: Vec<char> = line.chars().collect();

    encoded.push('"');
    for c in chars {
        match c {
            '"' => {
                encoded.push('\\');
                encoded.push('"');
            }
            '\\' => {
                encoded.push('\\');
                encoded.push('\\');
            }
            _ => encoded.push(c),
        }
    }
    encoded.push('"');

    String::from_iter(encoded)
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/08/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 8,
        phase_1: None,
        phase_2: None,
    };

    let mut representation_length = 0;
    let mut memory_length = 0;
    let mut encoded_length = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        representation_length += line.len();
        memory_length += count_memory(&line);
        encoded_length += encode(&line).len();
    }
    // println!("e: {}, r: {}, m: {}", encoded_length, representation_length, memory_length);
    res.phase_1 = Some(AocResultType::Usize(representation_length - memory_length));
    res.phase_2 = Some(AocResultType::Usize(encoded_length - representation_length));

    res
}
