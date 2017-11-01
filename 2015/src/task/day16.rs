use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
struct Aunt {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

struct Criteria {
    children: usize,
    cats: usize,
    samoyeds: usize,
    pomeranians: usize,
    akitas: usize,
    vizslas: usize,
    goldfish: usize,
    trees: usize,
    cars: usize,
    perfumes: usize,
}

impl Aunt {
    fn new(spec: &str) -> Aunt {
        let tokens: Vec<&str> = spec.split(|c| c == ' ' || c == ',' || c == ':')
            .collect();
        let num = usize::from_str(tokens[1]).unwrap();
        let mut a = Aunt {
            number: num,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };

        for (i, t) in tokens.iter().enumerate() {
            match *t {
                "children" => a.children = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "cats" => a.cats = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "samoyeds" => a.samoyeds = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "pomeranians" => a.pomeranians = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "akitas" => a.akitas = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "vizslas" => a.vizslas = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "goldfish" => a.goldfish = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "trees" => a.trees = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "cars" => a.cars = Some(usize::from_str(tokens[i + 2]).unwrap()),
                "perfumes" => a.perfumes = Some(usize::from_str(tokens[i + 2]).unwrap()),
                _ => {}
            }
        }
        a
    }
}

impl Criteria {
    fn matches(&self, aunt: &Aunt) -> bool {
        (match aunt.children {
             None => true,
             Some(x) => x == self.children, 
         }) &
        (match aunt.cats {
             None => true,
             Some(x) => x == self.cats,
         }) &
        (match aunt.samoyeds {
             None => true,
             Some(x) => x == self.samoyeds,
         }) &
        (match aunt.pomeranians {
             None => true,
             Some(x) => x == self.pomeranians,
         }) &
        (match aunt.akitas {
             None => true,
             Some(x) => x == self.akitas,
         }) &
        (match aunt.vizslas {
             None => true,
             Some(x) => x == self.vizslas,
         }) &
        (match aunt.goldfish {
             None => true,
             Some(x) => x == self.goldfish,
         }) &
        (match aunt.trees {
             None => true,
             Some(x) => x == self.trees,
         }) &
        (match aunt.cars {
             None => true,
             Some(x) => x == self.cars,
         }) &
        (match aunt.perfumes {
             None => true,
             Some(x) => x == self.perfumes,
         })
    }
    fn range_matches(&self, aunt: &Aunt) -> bool {
        (match aunt.children {
             None => true,
             Some(x) => x == self.children, 
         }) &
        (match aunt.cats {
             None => true,
             Some(x) => x > self.cats,
         }) &
        (match aunt.samoyeds {
             None => true,
             Some(x) => x == self.samoyeds,
         }) &
        (match aunt.pomeranians {
             None => true,
             Some(x) => x < self.pomeranians,
         }) &
        (match aunt.akitas {
             None => true,
             Some(x) => x == self.akitas,
         }) &
        (match aunt.vizslas {
             None => true,
             Some(x) => x == self.vizslas,
         }) &
        (match aunt.goldfish {
             None => true,
             Some(x) => x < self.goldfish,
         }) &
        (match aunt.trees {
             None => true,
             Some(x) => x > self.trees,
         }) &
        (match aunt.cars {
             None => true,
             Some(x) => x == self.cars,
         }) &
        (match aunt.perfumes {
             None => true,
             Some(x) => x == self.perfumes,
         })
    }
}

fn load_file(file_name: &str) -> Vec<Aunt> {
    let mut aunts: Vec<Aunt> = Vec::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        aunts.push(Aunt::new(&line));
    }

    aunts
}

fn search_criteria() -> Criteria {
    Criteria {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    }
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/16/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let aunts = load_file(&full_name);
    let criteria = search_criteria();
    let mut phase_1_match = 0;
    let mut phase_2_match = 0;
    for a in aunts {
        if criteria.matches(&a) {
            // println!("MATCH {:?}", a);
            phase_1_match = a.number;
        }
        if criteria.range_matches(&a) {
            // println!("RANGE MATCH {:?}", a);
            phase_2_match = a.number;
        }
    }

    let res: AocResult = AocResult {
        day: 16,
        phase_1: Some(AocResultType::Usize(phase_1_match)),
        phase_2: Some(AocResultType::Usize(phase_2_match)),
    };

    res
}
