use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

fn caesar(code: &String, key: i32) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();

    let mut decrypted = String::with_capacity(50);
    for c in code.chars() {
        if c == '-' {
            decrypted.push(' ');
        } else {
            for idx in 0..alphabet.len() {
                if c == alphabet[idx] {
                    let new_index = (idx + key as usize) % alphabet.len();
                    decrypted.push(alphabet[new_index]);
                    break;
                }
            }
        }
    }
    
    return decrypted;
}

fn check_code(code: &String) -> i32 {

    let mut code_map = BTreeMap::new();
    let mut code_id = String::with_capacity(5);
    let mut code_cs = String::with_capacity(5);
    let mut orig_code = String::with_capacity(50);
    let mut in_id = false;
    let mut in_cs = false;
        
    for c in code.chars() {
        if c.is_numeric() {
            in_id = true;
        }
        if in_id && c == '[' {
            in_cs = true;
            in_id = false;
            continue;
        }
        if in_cs && c == ']' {
            break;
        }
        if in_id {
            code_id.push(c);
            continue;
        }
        if in_cs {
            code_cs.push(c);
            continue;
        }
        orig_code.push(c);
        if c == '-' {
            continue;
        }
        if code_map.contains_key(&c) {
            if let Some(v) = code_map.get_mut(&c) {
                *v = *v + 1;
            }
        } else {
            code_map.insert(c, 1);
        }
    }

    let mut rev_map = BTreeMap::new();
    
    for (key, value) in code_map.iter() {
        if rev_map.contains_key(&value) {
            if let Some(v) = rev_map.get_mut(&value) as Option<&mut Vec<&char>> {
                v.push(key);
            }
        } else {
            rev_map.insert(value, vec![key]);
        }
    }

    let mut checksum = String::with_capacity(5);
    let mut i = 0;
    'outer: for (_, value) in rev_map.iter().rev() {
        for c in value {
            checksum.push(**c);
            i = i + 1;
            if i == 5 {
                break 'outer;
            }
        }
    }
    //println!("calculated: {}, in code: {}, id: {}", checksum, code_cs, code_id);

    let mut room_id = 0;
    if checksum == code_cs {
        room_id = code_id.parse().unwrap();
        let decrypted_code = caesar(&orig_code, room_id);
        if decrypted_code.contains("north") {
            println!("{} -> {}", decrypted_code, room_id);
        }
    }
    
    return room_id;
}

fn main() {

    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut csum = 0;
    for line in reader.lines() {
        let code = line.unwrap();
        let cc = check_code(&code);
        csum = csum + cc;
    }
    println!("Sum of valid ids: {}", csum);
}
