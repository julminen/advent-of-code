use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;
use common;
use common::AocResult;
use common::AocResultType;

mod direct_gate;
mod not_gate;
mod and_gate;
mod or_gate;
mod lshift_gate;
mod rshift_gate;

#[derive(Clone, Debug)]
enum Input {
    Direct(u16),
    Indirect(String),
}

struct Memory {
    mem: BTreeMap<String, u16>,
}

impl Memory {
    fn new() -> Memory {
        Memory { mem: BTreeMap::new() }
    }
    fn get(&self, key: &Input) -> Option<u16> {
        match key {
            &Input::Direct(x) => Some(x),
            &Input::Indirect(ref x) => {
                if self.mem.contains_key(x) {
                    Some(self.mem.get(x).unwrap().clone())
                } else {
                    None
                }
            }
        }
    }
    fn set(&mut self, key: String, value: u16) {
        self.mem.insert(key, value);
    }
    fn clear(&mut self) {
        self.mem.clear();
    }
}

trait Gate {
    fn operate(&self, mem: &mut Memory) -> bool;
    fn get_output_name(&self) -> &str;
    fn get_input_names(&self) -> Vec<String>;
    fn to_string(&self) -> String;
}




pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/07/");
    let mut target = String::from("a");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
            target = common::get_input("Check register", &target);
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 7,
        phase_1: None,
        phase_2: None,
    };

    let mut gates: Vec<(bool, Box<Gate>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let cmd: Vec<&str> = line.split_whitespace().collect();
        let g: Box<Gate> = if cmd[0] == "NOT" {
            Box::new(not_gate::NotGate::new(&cmd))
        } else {
            match cmd[1] {
                "->" => Box::new(direct_gate::DirectGate::new(&cmd)),
                "AND" => Box::new(and_gate::AndGate::new(&cmd)),
                "OR" => Box::new(or_gate::OrGate::new(&cmd)),
                "LSHIFT" => Box::new(lshift_gate::LShiftGate::new(&cmd)),
                "RSHIFT" => Box::new(rshift_gate::RShiftGate::new(&cmd)),
                _ => panic!("Unknown command {}", line),
            }
        };
        gates.push((false, g));
    }

    let mut memory = Memory::new();

    // Phase 1: solve all, return register
    let mut changes = true;
    while changes {
        changes = false;
        for gi in 0..gates.len() {
            if gates[gi].0 == false {
                let opr = gates[gi].1.operate(&mut memory);
                gates[gi].0 = opr;
                changes = opr || changes;
            }
        }
    }

    let a = memory.get(&Input::Indirect(target.clone()));
    res.phase_1 = match a {
        None => None,
        Some(x) => Some(AocResultType::U16(x)),
    };

    // Phase 2: reset memory, override b with a value from previous step
    match a {
        None => println!("Skipping phase 2"),
        Some(a) => {
            memory.clear();
            for gi in 0..gates.len() {
                gates[gi].0 = false;
                if gates[gi].1.get_output_name() == "b" {
                    let ip = a.to_string();
                    let mut cmd: Vec<&str> = Vec::with_capacity(3);
                    cmd.push(&ip);
                    cmd.push("->");
                    cmd.push("b");
                    gates[gi].1 = Box::new(direct_gate::DirectGate::new(&cmd));
                }
            }
            let mut changes = true;
            while changes {
                changes = false;
                for gi in 0..gates.len() {
                    if gates[gi].0 == false {
                        let opr = gates[gi].1.operate(&mut memory);
                        gates[gi].0 = opr;
                        changes = opr || changes;
                    }
                }
            }

            let a = memory.get(&Input::Indirect(target));
            res.phase_2 = match a {
                None => None,
                Some(x) => Some(AocResultType::U16(x)),
            };
        }
    }


    res
}


#[test]
fn test_direct_gate() {

    let decl: Vec<&str> = "44815 => output".split_whitespace().collect();
    let g = direct_gate::DirectGate::new(&decl);

    let mut m = Memory::new();

    let output = Input::Indirect("output".to_string());
    assert_eq!(m.get(&output), None);
    g.operate(&mut m);
    assert_eq!(m.get(&output), Some(44815));
}
#[test]
fn test_not_gate() {
    let decl: Vec<&str> = "NOT input -> output".split_whitespace().collect();

    let out_key_name = "output";
    let in_key_name = "input";
    let out_key = Input::Indirect(out_key_name.to_string());
    let in_key = Input::Indirect(in_key_name.to_string());

    let mut m = Memory::new();
    m.set(in_key_name.to_string(), 0b0011_1100_1010_0101);

    let g = not_gate::NotGate::new(&decl);

    assert_eq!(m.get(&in_key), Some(0b0011_1100_1010_0101));
    assert_eq!(m.get(&out_key), None);
    g.operate(&mut m);
    assert_eq!(m.get(&out_key), Some(0b1100_0011_0101_1010));
}

#[test]
fn test_and_gate() {

    // 44815 = 0xaf0f = 0b1010_1111_0000_1111
    let decl: Vec<&str> = "in AND 44815 => output".split_whitespace().collect();
    let g = and_gate::AndGate::new(&decl);

    let mut m = Memory::new();
    m.set("in".to_string(), 0b0011_0011_0011_0011);

    let output = Input::Indirect("output".to_string());
    assert_eq!(m.get(&Input::Indirect("in".to_string())), Some(0x3333));
    assert_eq!(m.get(&output), None);
    g.operate(&mut m);
    assert_eq!(m.get(&output), Some(0b0010_0011_0000_0011));
}

#[test]
fn test_or_gate() {

    // 44815 = 0xaf0f = 0b1010_1111_0000_1111
    let decl: Vec<&str> = "in OR 44815 => output".split_whitespace().collect();
    let g = or_gate::OrGate::new(&decl);

    let mut m = Memory::new();
    m.set("in".to_string(), 0b0011_0011_0011_0011);

    let output = Input::Indirect("output".to_string());
    assert_eq!(m.get(&Input::Indirect("in".to_string())), Some(0x3333));
    assert_eq!(m.get(&output), None);
    g.operate(&mut m);
    assert_eq!(m.get(&output), Some(0b1011_1111_0011_1111));
}

#[test]
fn test_lshift_gate() {
    let decl: Vec<&str> = "in LSHIFT 3 => output".split_whitespace().collect();
    let g = lshift_gate::LShiftGate::new(&decl);

    let mut m = Memory::new();
    m.set("in".to_string(), 0b1010_1111_0000_1111);

    let output = Input::Indirect("output".to_string());
    assert_eq!(m.get(&Input::Indirect("in".to_string())),
               Some(0b1010_1111_0000_1111));
    assert_eq!(m.get(&output), None);
    g.operate(&mut m);
    assert_eq!(m.get(&output), Some(0b0_1111_0000_1111_000));
}

#[test]
fn test_rshift_gate() {
    let decl: Vec<&str> = "in RSHIFT 3 => output".split_whitespace().collect();
    let g = rshift_gate::RShiftGate::new(&decl);

    let mut m = Memory::new();
    m.set("in".to_string(), 0b1010_1111_0000_1111);

    let output = Input::Indirect("output".to_string());
    assert_eq!(m.get(&Input::Indirect("in".to_string())),
               Some(0b1010_1111_0000_1111));
    assert_eq!(m.get(&output), None);
    g.operate(&mut m);
    assert_eq!(m.get(&output), Some(0b000_1010_1111_0000_1));
}
