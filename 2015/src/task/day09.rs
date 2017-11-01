use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;
use common;
use common::AocResultType;
use common::AocResult;

struct Map {
    places: Vec<String>,
    distances: Vec<Vec<usize>>,
}

impl Map {
    fn new(file_name: &str) -> Map {
        let reader = BufReader::new(File::open(file_name).unwrap());
        let mut places: Vec<String> = Vec::new();
        let mut dist_map: BTreeMap<(usize, usize), usize> = BTreeMap::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() == 0 {
                continue;
            }
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let from = tokens[0].to_string();
            let to = tokens[2].to_string();
            let distance: usize = tokens[4].parse().unwrap();

            if !places.contains(&from) {
                places.push(from.clone());
            }
            let from_idx = places.iter().position(|x| x == &from).unwrap();

            if !places.contains(&to) {
                places.push(to.clone());
            }
            let to_idx = places.iter().position(|x| x == &to).unwrap();
            dist_map.insert((from_idx, to_idx), distance);
            dist_map.insert((to_idx, from_idx), distance);
        }

        let mut distances: Vec<Vec<usize>> = vec![vec![0; places.len()]; places.len()];
        for (k, v) in dist_map {
            distances[k.0][k.0] = v;
            distances[k.1][k.0] = v;
        }

        Map {
            places: places,
            distances: distances,
        }
    }

    fn get_distance(&self, path: &Vec<usize>) -> usize {
        let mut dist = 0;
        for i in 1..path.len() {
            dist += self.distances[path[i - 1]][path[i]];
        }
        dist
    }
}

// Permutate vector of usizes. Return true if new permutation was found
fn permutate(path: &mut Vec<usize>) -> bool {
    let mut k = 0;
    let mut l = 0;
    let mut p_found = false;

    for i in (1..path.len()).rev() {
        if path[i - 1] < path[i] {
            k = i - 1;
            p_found = true;
            break;
        }
    }
    if p_found {
        for i in (k..path.len()).rev() {
            if path[k] < path[i] {
                l = i;
                break;
            }
        }
        path.swap(k, l);
        let mut b = path.len();
        for a in (k + 1)..path.len() {
            b = b - 1;
            if b > a {
                path.swap(a, b);
            } else {
                break;
            }
        }
    }

    p_found
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/09/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }


    let mut res: AocResult = AocResult {
        day: 9,
        phase_1: None,
        phase_2: None,
    };

    let map = Map::new(&full_name);
    let mut path: Vec<usize> = (0..map.places.len()).collect();
    let mut shortest_path = path.clone();
    let mut shortest_distance = usize::max_value();
    let mut longest_path = path.clone();
    let mut longest_distance = 0;
    loop {
        let dist = map.get_distance(&path);
        if dist < shortest_distance {
            shortest_distance = dist;
            shortest_path = path.clone();
        }
        if dist > longest_distance {
            longest_distance = dist;
            longest_path = path.clone();
        }
        if !permutate(&mut path) {
            break;
        }
    }

    print!("Shortest path: ");
    for p in 0..shortest_path.len() - 1 {
        print!("{} -> ", map.places[shortest_path[p]]);
    }
    println!("{} = {}",
             map.places[*shortest_path.last().unwrap()],
             shortest_distance);

    print!("Longest path: ");
    for p in 0..longest_path.len() - 1 {
        print!("{} -> ", map.places[longest_path[p]]);
    }
    println!("{} = {}",
             map.places[*longest_path.last().unwrap()],
             longest_distance);

    res.phase_1 = Some(AocResultType::Usize(shortest_distance));
    res.phase_2 = Some(AocResultType::Usize(longest_distance));

    res
}
