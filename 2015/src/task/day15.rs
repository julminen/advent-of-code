use common;
use common::AocResultType;
use common::AocResult;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;
use std::cmp;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: usize,
}

impl Ingredient {
    fn new(spec: &str) -> Ingredient {
        let tokens: Vec<&str> = spec.split(|c| c == ' ' || c == ',' || c == ':')
            .collect();
        Ingredient {
            name: tokens[0].to_string(),
            capacity: isize::from_str(tokens[3]).unwrap(),
            durability: isize::from_str(tokens[6]).unwrap(),
            flavor: isize::from_str(tokens[9]).unwrap(),
            texture: isize::from_str(tokens[12]).unwrap(),
            calories: usize::from_str(tokens[15]).unwrap(),
        }
    }
}

fn score(ingredients: &Vec<Ingredient>, combination: &Vec<usize>) -> (usize, usize) {
    if ingredients.len() != combination.len() {
        panic!("Combinations and ingredients length do not match");
    }
    let len = ingredients.len();
    let mut cap_score = 0;
    let mut dur_score = 0;
    let mut fla_score = 0;
    let mut tex_score = 0;
    let mut calories = 0;

    for i in 0..len {
        cap_score += ingredients[i].capacity * combination[i] as isize;
        dur_score += ingredients[i].durability * combination[i] as isize;
        fla_score += ingredients[i].flavor * combination[i] as isize;
        tex_score += ingredients[i].texture * combination[i] as isize;
        calories += ingredients[i].calories * combination[i];
    }

    (cmp::max(cap_score, 0) as usize * cmp::max(dur_score, 0) as usize *
     cmp::max(fla_score, 0) as usize * cmp::max(tex_score, 0) as usize,
     calories)
}

// Recursive search for combinations
fn nonzero_combos(sum: usize, elements: usize) -> Vec<Vec<usize>> {
    if elements > sum {
        panic!("Too many elements");
    }
    if elements < 1 {
        panic!("Too few elements!");
    }
    let max = sum - (elements - 1) + 1;
    let mut combos = Vec::new();
    if elements == 1 {
        combos.push(vec![max - 1]);
    } else {
        for i in 1..max {
            let sub = nonzero_combos(sum - i, elements - 1);
            for mut s in sub {
                let mut res: Vec<usize> = Vec::new();
                res.push(i);
                res.append(&mut s);
                combos.push(res);
            }
        }
    }

    combos
}

fn load_file(file_name: &str) -> Vec<Ingredient> {
    let mut ingredients: Vec<Ingredient> = Vec::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        ingredients.push(Ingredient::new(&line));
    }

    ingredients
}


pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/15/");
    let spoons = 100;
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let ingredients = load_file(&full_name);
    for i in &ingredients {
        println!("{:?}", i);
    }

    let combos = nonzero_combos(spoons, ingredients.len());
    println!("{} combinations for {} spoons of {} ingredients",
             combos.len(),
             spoons,
             ingredients.len());
    let mut max_score = 0;
    let mut max_combo = Vec::new();

    let mut c_score = 0;
    let mut c_combo = Vec::new();

    for c in combos {
        let (score, calories) = score(&ingredients, &c);
        if max_score < score {
            max_score = score;
            max_combo.truncate(0);
            max_combo.append(&mut c.clone());
        }
        if calories == 500 && c_score < score {
            c_score = score;
            c_combo.truncate(0);
            c_combo.append(&mut c.clone());
        }
    }
    println!("MAX combination: {:?}: {} points", max_combo, max_score);
    println!("500 cal limit  : {:?}: {} points", c_combo, c_score);

    let res: AocResult = AocResult {
        day: 15,
        phase_1: Some(AocResultType::Usize(max_score)),
        phase_2: Some(AocResultType::Usize(c_score)),
    };

    res
}

#[test]
fn combo_one() {
    let ones = nonzero_combos(10, 1);
    let ones_test: Vec<Vec<usize>> = vec![vec![10]];
    assert_eq!(ones, ones_test);
}

#[test]
fn combo_two() {
    let c = nonzero_combos(10, 2);
    let d: Vec<Vec<usize>> = vec![vec![1, 9], vec![2, 8], vec![3, 7], vec![4, 6], vec![5, 5],
                                  vec![6, 4], vec![7, 3], vec![8, 2], vec![9, 1]];
    assert_eq!(c, d);
}

#[test]
fn combo_three() {
    let c = nonzero_combos(5, 3);
    let d: Vec<Vec<usize>> = vec![vec![1, 1, 3],
                                  vec![1, 2, 2],
                                  vec![1, 3, 1],
                                  vec![2, 1, 2],
                                  vec![2, 2, 1],
                                  vec![3, 1, 1]];
    assert_eq!(c, d);
}

#[test]
fn combo_four() {
    let c = nonzero_combos(8, 4);
    let d: Vec<Vec<usize>> = vec![vec![1, 1, 1, 5],
                                  vec![1, 1, 2, 4],
                                  vec![1, 1, 3, 3],
                                  vec![1, 1, 4, 2],
                                  vec![1, 1, 5, 1],
                                  vec![1, 2, 1, 4],
                                  vec![1, 2, 2, 3],
                                  vec![1, 2, 3, 2],
                                  vec![1, 2, 4, 1],
                                  vec![1, 3, 1, 3],
                                  vec![1, 3, 2, 2],
                                  vec![1, 3, 3, 1],
                                  vec![1, 4, 1, 2],
                                  vec![1, 4, 2, 1],
                                  vec![1, 5, 1, 1],
                                  vec![2, 1, 1, 4],
                                  vec![2, 1, 2, 3],
                                  vec![2, 1, 3, 2],
                                  vec![2, 1, 4, 1],
                                  vec![2, 2, 1, 3],
                                  vec![2, 2, 2, 2],
                                  vec![2, 2, 3, 1],
                                  vec![2, 3, 1, 2],
                                  vec![2, 3, 2, 1],
                                  vec![2, 4, 1, 1],
                                  vec![3, 1, 1, 3],
                                  vec![3, 1, 2, 2],
                                  vec![3, 1, 3, 1],
                                  vec![3, 2, 1, 2],
                                  vec![3, 2, 2, 1],
                                  vec![3, 3, 1, 1],
                                  vec![4, 1, 1, 2],
                                  vec![4, 1, 2, 1],
                                  vec![4, 2, 1, 1],
                                  vec![5, 1, 1, 1]];
    assert_eq!(c.len(), c.len());

    for i in 0..c.len() {
        println!("{}: {:?}  :  {:?}", i, c[i], d[i]);
        assert_eq!(c[i], d[i]);
    }

    assert_eq!(c, d);
}

#[test]
fn combo_cent() {
    let c = nonzero_combos(100, 4);
    for v in c {
        assert_eq!(v.len(), 4);
        assert_eq!(v.iter().fold(0, |acc, &x| acc + x), 100);
    }
}
