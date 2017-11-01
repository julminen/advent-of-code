use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use common;
use common::AocResultType;
use common::AocResult;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Replacer {
    replaced: Vec<u8>,
    replacement: Vec<u8>,
}

impl Replacer {
    fn new(spec: &str) -> Option<Replacer> {
        let tokens: Vec<&str> = spec.split(" => ").collect();
        if tokens.len() == 2 {
            Some(Replacer {
                     replaced: tokens[0].as_bytes().to_vec(),
                     replacement: tokens[1].as_bytes().to_vec(),
                 })
        } else {
            None
        }
    }
}
impl fmt::Display for Replacer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} => {}",
               String::from_utf8(self.replaced.clone()).unwrap(),
               String::from_utf8(self.replacement.clone()).unwrap())
    }
}

type Molecule = Vec<u8>;

fn load_file(file_name: &str) -> (Vec<Replacer>, Molecule) {
    let mut res: Vec<Replacer> = Vec::new();
    let mut molecule = Vec::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        match Replacer::new(&line) {
            None => molecule = line.as_bytes().to_vec(),
            Some(r) => res.push(r),
        }
    }

    (res, molecule)
}

// Not really needed since only n used was 1
fn replace_str_n(str: &Vec<u8>, from: &Vec<u8>, to: &Vec<u8>, n: usize) -> Option<Vec<u8>> {
    if from.len() > 0 && from.len() <= str.len() {
        let mut x = 0;
        'main: for i in 0..str.len() - (from.len() - 1) {
            for j in 0..from.len() {
                if str[i + j] != from[j] {
                    continue 'main;
                }
            }
            // Match
            x += 1;
            if x < n {
                continue 'main;
            }
            let mut target: Vec<u8> = Vec::with_capacity(str.len() + to.len() - from.len());
            for j in 0..i {
                target.push(str[j]);
            }
            for j in 0..to.len() {
                target.push(to[j]);
            }
            for j in (i + from.len())..str.len() {
                target.push(str[j]);
            }
            return Some(target);
        }
    }

    None
}

fn get_new_molecules(replacers: &Vec<Replacer>, molecule: &Molecule) -> BTreeSet<Molecule> {
    let mut molecule_set: BTreeSet<Molecule> = BTreeSet::new();

    for r in replacers {
        let mut n = 1;
        let from = &r.replaced;
        let to = &r.replacement;
        loop {
            match replace_str_n(molecule, from, to, n) {
                Some(some) => {
                    molecule_set.insert(some);
                } 
                None => break,
            }
            n += 1;
        }
    }

    molecule_set
}

fn calibrate(replacers: &Vec<Replacer>, molecule: &Molecule) -> usize {
    let molecule_set = get_new_molecules(replacers, molecule);

    molecule_set.len()
}

fn apply_rev_replace(replacers: &Vec<Replacer>, molecule: &Molecule) -> Option<Molecule> {
    let mut m = None;
    for r in replacers {
        let from = &r.replacement;
        let to = &r.replaced;
        m = replace_str_n(molecule, from, to, 1);
        if m != None {
            break;
        }
    }

    m
}

// Had to get some help for this from https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
// after brute force search just exploded
/*
Valid molecule is produced by 
1) e => XX or X => XX
2) X => X Rn X Ar or X => X Rn X Y X Ar or X => X Rn X Y X Y X Ar

If you replace "Rn Y Ar" with "( , )" task starts to seem easier
*/


fn count_create_steps(replacers: &Vec<Replacer>, molecule: &Molecule) -> usize {
    let mut steps = 0;
    let mut mol: Option<Molecule> = Some(molecule.clone());
    loop {
        let pm = &mol.unwrap();
        mol = apply_rev_replace(replacers, &pm);
        if mol != None {
            steps += 1;
            println!("{}", String::from_utf8(mol.clone().unwrap()).unwrap());
        } else {
            if pm.len() > 1 {
                println!("Could not handle {}",
                         String::from_utf8(pm.clone()).unwrap());
            }
            break;
        }
    }

    steps
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/19/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let (replacers, molecule) = load_file(&full_name);

    //println!("Replacers:");
    //for r in &replacers {
    //    println!("{}", r);
    //}

    //println!("Molecule: {}", String::from_utf8(molecule.clone()).unwrap());

    let calib = calibrate(&replacers, &molecule);
    //println!("Calibration result: {}", calib);

    let steps = count_create_steps(&replacers, &molecule);
    //println!("Steps to create target molecule: {}", steps);

    let res: AocResult = AocResult {
        day: 19,
        phase_1: Some(AocResultType::Usize(calib)),
        phase_2: Some(AocResultType::Usize(steps)),
    };

    res
}
