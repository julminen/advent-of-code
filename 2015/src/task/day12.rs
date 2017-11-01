extern crate rustc_serialize;
use self::rustc_serialize::json;

use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn sum_numbers(file_name: &str) -> i64 {
    let mut sum = 0;
    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let mut in_num = false;
        let mut num = String::from("");
        let line = line.unwrap();
        for c in line.chars() {
            match c {
                '-' => {
                    in_num = true;
                    num.push(c);
                }
                '0'...'9' => {
                    if !in_num {
                        in_num = true;
                    }
                    num.push(c);
                }
                _ => {
                    if in_num {
                        use std::str::FromStr;
                        let new_num = i64::from_str(&num).unwrap();
                        num.clear();
                        sum += new_num;
                        in_num = false;
                    }
                }
            }
        }
    }
    sum
}

pub fn sum_object(o: &json::Object) -> i64 {
    let mut sum = 0;
    for (_, value) in o {
        match value.as_string() {
            Some(s) => {
                if s == "red" {
                    sum = 0;
                    break;
                }
            }
            None => {
                sum += handle_json(value);
            }
        }
    }

    sum
}

pub fn sum_array(a: &json::Array) -> i64 {
    let mut sum = 0;
    for j in a {
        sum += handle_json(j);
    }
    sum
}

pub fn handle_json(j: &json::Json) -> i64 {
    match *j {
        json::Json::I64(n) => n,
        json::Json::U64(n) => n as i64,
        json::Json::F64(_) => 0,
        json::Json::String(_) => 0,
        json::Json::Boolean(_) => 0,
        json::Json::Array(ref n) => sum_array(n),
        json::Json::Object(ref n) => sum_object(n),
        json::Json::Null => 0,
    }
}

pub fn skip_reds(file_name: &str) -> i64 {
    let mut sum = 0;
    let mut reader = BufReader::new(File::open(file_name).unwrap());
    if let Ok(json) = json::Json::from_reader(&mut reader) {
        sum = handle_json(&json);
    } else {
        println!("fail");
    }


    sum
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/12/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let sum = sum_numbers(&full_name);
    let noreds = skip_reds(&full_name);

    let res: AocResult = AocResult {
        day: 12,
        phase_1: Some(AocResultType::Isize(sum as isize)),
        phase_2: Some(AocResultType::Isize(noreds as isize)),
    };

    res
}

#[test]
fn test_json_1_phase_1() {
    let file = String::from("input/12/test_1.json");
    let test = sum_numbers(&file);
    assert_eq!(test, 6);
}
#[test]
fn test_json_1_phase_2() {
    let file = String::from("input/12/test_1.json");
    let test = skip_reds(&file);
    assert_eq!(test, 6);
}

#[test]
fn test_json_2_phase_1() {
    let file = String::from("input/12/test_2.json");
    let test = sum_numbers(&file);
    assert_eq!(test, 6);
}
#[test]
fn test_json_2_phase_2() {
    let file = String::from("input/12/test_2.json");
    let test = skip_reds(&file);
    assert_eq!(test, 4);
}

#[test]
fn test_json_3_phase_1() {
    let file = String::from("input/12/test_3.json");
    let test = sum_numbers(&file);
    assert_eq!(test, 15);
}
#[test]
fn test_json_3_phase_2() {
    let file = String::from("input/12/test_3.json");
    let test = skip_reds(&file);
    assert_eq!(test, 0);
}

#[test]
fn test_json_4_phase_1() {
    let file = String::from("input/12/test_4.json");
    let test = sum_numbers(&file);
    assert_eq!(test, 6);
}
#[test]
fn test_json_4_phase_2() {
    let file = String::from("input/12/test_4.json");
    let test = skip_reds(&file);
    assert_eq!(test, 6);
}

#[test]
fn test_json_5_phase_1() {
    let file = String::from("input/12/test_5.json");
    let test = sum_numbers(&file);
    assert_eq!(test, 15);
}
#[test]
fn test_json_5_phase_2() {
    let file = String::from("input/12/test_5.json");
    let test = skip_reds(&file);
    assert_eq!(test, 15);
}
