use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive (Debug,Clone)]
struct Person {
    name: String,
    relations: BTreeMap<String, isize>,
}

impl Person {
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
            relations: BTreeMap::new(),
        }
    }
    fn add_relation(&mut self, name: &str, relation: isize) {
        self.relations.insert(name.to_string(), relation);
    }
    fn happiness(&self, left: &str, right: &str) -> isize {
        let left_happiness = match self.relations.get(left) {
            Some(h) => h,
            None => panic!("Unknown neighbor '{}' to '{}'", left, self.name),
        };
        let right_happiness = match self.relations.get(right) {
            Some(h) => h,
            None => panic!("Unknown neighbor '{}' to '{}'", right, self.name),
        };

        left_happiness + right_happiness
    }
}

fn load_file(file_name: &str) -> BTreeMap<String, Person> {
    let mut persons: BTreeMap<String, Person> = BTreeMap::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(|c| c == ' ' || c == '.').collect();
        if tokens.len() < 11 {
            println!("Skipping line '{}'", line);
            continue;
        }
        let s = tokens[0];
        let o = tokens[10];
        let v = isize::from_str(tokens[3]).unwrap() *
                match tokens[2] {
                    "gain" => 1,
                    "lose" => -1,
                    _ => panic!("Unknown relation {} at {}", tokens[2], line),
                };
        if !persons.contains_key(s) {
            persons.insert(s.to_string(), Person::new(s));
        }

        match persons.get_mut(s) {
            Some(p) => {
                p.add_relation(o, v);
            }
            None => panic!("Inserted person is lost"),
        }
    }

    persons
}

fn happiness(order: &Vec<String>, people: &BTreeMap<String, Person>) -> isize {
    let mut happiness = 0;
    let count = order.len();
    for i in 0..count {
        let left_index = (i + count - 1) % count;
        let right_index = (i + 1) % count;
        happiness += people
            .get(&order[i])
            .unwrap()
            .happiness(&order[left_index], &order[right_index]);
    }
    happiness
}

// Lexicographic permutations (except, keep first item in place)
pub fn permutate(names: &mut Vec<String>) -> bool {
    let mut k = 0;
    // find largest index k such that a[k] < a[k+1]
    for i in (0..names.len() - 1).rev() {
        if names[i] < names[i + 1] {
            k = i;
            break;
        }
    }
    if k == 0 {
        // Not handling last change - this is last permutation
        return false;
    }
    // find largest index l so that a[k] < a[l]
    let mut l = 0;
    for i in (k..names.len()).rev() {
        if names[k] < names[i] {
            l = i;
            break;
        }
    }
    assert!(l > k);
    // Swap k with l
    names.swap(k, l);
    // Reverse sequence from a[k+1] to last element
    names.split_at_mut(k + 1).1.reverse();

    true
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/13/");
    let mut debug = false;
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
            debug = true;
        }
    }

    let mut people = load_file(&full_name);
    let mut order: Vec<String> = people.keys().cloned().collect();
    let mut max_happiness = isize::min_value();
    let mut happy_order: Vec<String> = Vec::new();
    loop {
        let happiness = happiness(&order, &people);
        if happiness > max_happiness {
            max_happiness = happiness;
            happy_order.truncate(0);
            happy_order.extend(order.clone().into_iter());
        }
        if !permutate(&mut order) {
            break;
        }
    }

    if debug {
        println!("First order is {:?}", happy_order);
    }

    // Phase 2: add self
    let mut host = Person::new("Jussi");
    for other in &order {
        host.add_relation(&other, 0);
        let mut n = people.get_mut(other).unwrap();
        n.add_relation(&host.name, 0);
    }
    order.push(host.name.clone());
    people.insert(host.name.clone(), host);
    order.sort();

    let mut final_happiness = isize::min_value();
    loop {
        let happiness = happiness(&order, &people);
        if happiness > final_happiness {
            final_happiness = happiness;
            happy_order.truncate(0);
            happy_order.extend(order.clone().into_iter());
        }
        if !permutate(&mut order) {
            break;
        }
    }

    if debug {
        println!("Second order is {:?}", happy_order);
    }


    let res: AocResult = AocResult {
        day: 13,
        phase_1: Some(AocResultType::Isize(max_happiness)),
        phase_2: Some(AocResultType::Isize(final_happiness)),
    };

    res
}
