use std::io::{self};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Space,
    Poi { id: usize },
}

#[derive(Debug, Clone, Ord, Eq, PartialOrd, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Point {
    id: usize,
    location: Location,
}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.id == other.id
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Map {
    contents: Vec<Vec<Tile>>,
    poi: Vec<Point>,
    paths: Vec<BTreeMap<usize, Vec<Location>>>,
}

impl Map {
    fn new_from_file(file_name: String) -> Map {
        let reader = BufReader::new(File::open(file_name).unwrap());
        let mut contents: Vec<Vec<Tile>> = Vec::new();
        let mut points: Vec<Point> = Vec::new();

        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.len() == 0 {
                continue;
            }
            let mut row: Vec<Tile> = Vec::with_capacity(line.len());
            for (x, col) in line.chars().enumerate() {
                row.push(
                    match col {
                        '#' => Tile::Wall,
                        '.' => Tile::Space,
                        _ => {
                            let id: usize = col.to_string().parse().unwrap();
                            points.push(
                                Point {
                                    id: id,
                                    location: Location { x: x, y: y },
                                }
                                );
                            Tile::Poi{ id: id }
                        }
                    }
                    );
            }
            contents.push(row);
        }
        points.sort();
        let paths = Vec::with_capacity(points.len());
        Map {
            contents: contents,
            poi: points,
            paths: paths,
        }
    }

    fn get_accessible_neighbors(&self, location: &Location) -> Vec<Location> {
        let x = location.x;
        let y = location.y;
        let mut neighbors = Vec::with_capacity(4);
        // left
        if x > 0 && self.contents[y][x-1] != Tile::Wall {
            neighbors.push(Location { x: x-1, y: y });
        }
        // right
        if x + 1 < self.contents[y].len() && self.contents[y][x+1] != Tile::Wall {
            neighbors.push(Location { x: x+1, y: y });
        }
        // up
        if y > 0 && self.contents[y-1][x] != Tile::Wall {
            neighbors.push(Location { x: x, y: y - 1 });
        }
        // down
        if y + 1 < self.contents.len() && self.contents[y + 1][x] != Tile::Wall {
            neighbors.push(Location { x: x, y: y + 1 });
        }
        neighbors
    }
    
    fn solve_paths(&mut self) {
        for id in &self.poi {
            //println!("paths for {:?}", id);
            let start = id;
            let start = start.location.clone();
            
            let mut came_from: BTreeMap<Location, Option<Location>> = BTreeMap::new();
            let mut frontier: Vec<Location> = Vec::new();
            
            frontier.push(start.clone());
            came_from.insert(start.clone(), None);

            while frontier.len() > 0 {
                let current: Vec<Location> = frontier.drain(0..).collect();
                for c in &current {
                    let neighbors = self.get_accessible_neighbors(c);
                    for n in &neighbors {
                        if ! came_from.contains_key(n) {
                            frontier.push(n.clone());
                            came_from.insert(n.clone(), Some(c.clone()));
                        }
                    }
                }
            }
            let mut node_paths: BTreeMap<usize, Vec<Location>>  = BTreeMap::new();
            for end in &self.poi {
                if end.location != start {
                    // Walk backwards
                    let mut path: Vec<Location> = Vec::new();
                    let mut current = end.location.clone();
                    while current != start {
                        path.push(current.clone());
                        let prev = came_from.get(&current);
                        match prev {
                            Some(c) => {
                                current = c.clone().unwrap();
                            },
                            None => {
                                println!("Premature end of path");
                                break;
                            },
                        }
                    }
                    //println!("Shortest path from {} to {}: {}",
                    //         id.id, end.id, path.len());
                    node_paths.insert(end.id, path);
                }
            }
            self.paths.push(node_paths);
        }
    }

}

fn get_input(prompt: &str, default: &str) -> String {
    print!("{} [{}]: ", prompt, default);
    match io::stdout().flush() {
        Ok(_) => {},
        Err(error) => println!("error: {}", error),
    }
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer = buffer.trim().to_string();
            if buffer == "" {
                buffer = default.to_string();
            }
        },
        Err(error) => println!("error: {}", error),
    }

    buffer
}

fn print_maze(map: &Map, path: &Vec<Location>) {
    let contents = &map.contents;
    for y in 0..contents.len() {
        for x in 0..contents[y].len() {
            let e = &contents[y][x];
            match *e {
                Tile::Wall => print!("█"),
                Tile::Space => {
                    if path.contains(&Location { x: x, y: y}) {
                        print!("o");
                    } else {
                        print!(" ")
                    }
                },
                Tile::Poi { id } => { print!("{}", id); },
            }
        }
        println!("");
    }
}

// Get next permutation
fn permutate(path: &mut Vec<usize>) -> bool {
    let mut k = 0;
    let mut l = 0;
    let mut p_found = false;
    
    for i in (1..path.len()).rev() {
        if path[i-1] < path[i] {
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
        for a in (k+1)..path.len() {
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

fn get_distance(path: &Vec<usize>, distances: &Vec<Vec<usize>>) -> usize {
    let mut dist: usize = 0;
    for p in 1..path.len() {
        let a = path[p-1];
        let b = path[p];
        dist += distances[a][b];
    }
    dist
}

fn find_shortest_path(start: usize, distances: &Vec<Vec<usize>>, visit: &Vec<usize>)
                      -> (Vec<usize>, usize)
{
    let mut shortest_path = Vec::new();
    let mut next_paths = visit.clone();
    next_paths.sort();
    next_paths.insert(0, start);
    let mut shortest_len = usize::max_value();
    loop {
        let len = get_distance(&next_paths, distances);
        if len < shortest_len {
            shortest_len = len;
            shortest_path = next_paths.clone();
        }
        // println!("Perm: {:?} len: {}", next_paths, len);
        if ! permutate(&mut next_paths) || next_paths[0] != start {
            break;
        }
    }
    (shortest_path, shortest_len)
}

fn find_shortest_loop(start: usize, distances: &Vec<Vec<usize>>, visit: &Vec<usize>)
                      -> (Vec<usize>, usize)
{
    let mut shortest_path = Vec::new();
    let mut next_paths = visit.clone();
    next_paths.sort();
    next_paths.insert(0, start);
    let mut shortest_len = usize::max_value();
    loop {
        next_paths.push(0);
        let len = get_distance(&next_paths, distances);
        if len < shortest_len {
            shortest_len = len;
            shortest_path = next_paths.clone();
        }
        next_paths.pop();
        // println!("Perm: {:?} len: {}", next_paths, len);
        if ! permutate(&mut next_paths) || next_paths[0] != start {
            break;
        }
    }
    (shortest_path, shortest_len)
}

fn main() {
    println!("AOC 24");

    let file_name = get_input("File name", "input");

    println!("Creating map from file '{}'", file_name);

    let mut map = Map::new_from_file(file_name);
    map.solve_paths();
    let node_count = map.paths.len();
    let mut distance_matrix: Vec<Vec<usize>> = vec![vec![0; node_count]; node_count];
    // map paths: Vec<BTreeMap<usize, Vec<Location>>>,
    for x in 0..node_count {
        for _ in 0..node_count {
            for p in &map.paths[x] {
                distance_matrix[x][*p.0] = p.1.len();
            }
        }
    }

    println!("Nodes: {}", node_count);

    println!("Distances:");
    print!("  ");
    for x in 0..node_count {
        print!(" {:3}", x);
    }
    println!("");
    for x in 0..node_count {
        print!("{}:", x);
        for y in 0..node_count {
            print!(" {:3}", distance_matrix[x][y]);
        }
        println!("");
    }

    let visit: Vec<usize> = (1..node_count).collect();
    let (nodes, length) = find_shortest_path(0, &distance_matrix, &visit);
    println!("Shortest path is {:?}: {}", nodes, length);

    let (loop_nodes, loop_length) = find_shortest_loop(0, &distance_matrix, &visit);
    println!("Shortest loop is {:?}: {}", loop_nodes, loop_length);
    
    let mut shortest_path = Vec::with_capacity(node_count);
    for n in 1..nodes.len() {
        let a = nodes[n-1];
        let b = nodes[n];
        let mut path: Vec<Location> = map.paths[a].get(&b).unwrap().clone();
        shortest_path.append(&mut path);
    }

    let mut shortest_loop = Vec::with_capacity(node_count + 1);
    for n in 1..loop_nodes.len() {
        let a = loop_nodes[n-1];
        let b = loop_nodes[n];
        let mut path: Vec<Location> = map.paths[a].get(&b).unwrap().clone();
        shortest_loop.append(&mut path);
    }

    println!("One way");
    print_maze(&map, &shortest_path);

    println!("Loop");
    print_maze(&map, &shortest_loop);
}
