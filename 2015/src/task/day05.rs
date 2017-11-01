use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use common;
use common::AocResultType;
use common::AocResult;

// nice 1: rule 1
fn nice_vowel_count(s: &str) -> bool {
    let mut vowels = 0;
    for c in s.chars() {
        vowels += match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 1,
            _ => 0,
        }
    }
    vowels >= 3
}

// nice 1: rule 2
fn has_doube_chars(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
        if chars[i - 1] == chars[i] {
            return true;
        }
    }
    false
}

// nice 1: rule 3
fn has_bad_substring(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

// nice 2: rule 1
fn has_non_overlapping_pair(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 1..chars.len() {
        for j in (i + 2)..chars.len() {
            if chars[i - 1] == chars[j - 1] && chars[i] == chars[j] {
                return true;
            }
        }
    }
    false
}

// nice 2: rule 2
fn has_repeating_letter(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 2..chars.len() {
        if chars[i - 2] == chars[i] {
            return true;
        }
    }
    false
}


pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/05/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 5,
        phase_1: None,
        phase_2: None,
    };

    let mut nice_count_1 = 0;
    let mut nice_count_2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        if nice_vowel_count(&line) && has_doube_chars(&line) && !has_bad_substring(&line) {
            nice_count_1 += 1;
        }
        if has_non_overlapping_pair(&line) && has_repeating_letter(&line) {
            nice_count_2 += 1;
        }
    }
    res.phase_1 = Some(AocResultType::Usize(nice_count_1));
    res.phase_2 = Some(AocResultType::Usize(nice_count_2));
    res
}
