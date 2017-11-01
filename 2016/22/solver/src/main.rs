use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct ServerMap {
    max_x: usize,
    max_y: usize,
    contents: BTreeMap<Point, Server>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Server {
    location: Point,
    size: usize,
    used: usize,
    avail: usize,
    contains_goal_data: bool,
}

impl Server {
    fn new(spec: &String) -> Server {
        let tokens: Vec<&str> = spec.split_whitespace().collect();
        let name: Vec<&str> = tokens[0].split('/').collect();
        let coords: Vec<&str> = name[3].split(
            |c| c == '-' || c == 'x' || c == 'y').collect();
        let x: usize = coords[2].parse().unwrap();
        let y: usize = coords[4].parse().unwrap();
        let size: usize = tokens[1].split('T').next().unwrap().parse().unwrap();
        let used: usize = tokens[2].split('T').next().unwrap().parse().unwrap();
        let avail: usize = tokens[3].split('T').next().unwrap().parse().unwrap();

        Server {
            location: Point { x: x, y: y },
            size: size,
            used: used,
            avail: avail,
            contains_goal_data: false,
        }
    }
   
    fn neighbors(&self, map: &ServerMap, walls: &HashSet<Point>) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        // left
        if self.location.x > 0 {
            let point = Point { x: self.location.x - 1, y: self.location.y };
            let other = map.contents.get(&point).unwrap();
            
            if ! walls.contains(&point) && other.used <= self.size {
                neighbors.push(point);
            }
        }
        // right
        if self.location.x < map.max_x {
            let point = Point { x: self.location.x + 1, y: self.location.y };
            let other = map.contents.get(&point).unwrap();

            if ! walls.contains(&point) && other.used <= self.size {
                neighbors.push(point);
            }
        }
        // up
        if self.location.y > 0 {
            let point = Point { x: self.location.x, y: self.location.y - 1};
            let other = map.contents.get(&point).unwrap();

            if ! walls.contains(&point) && other.used <= self.size {
                neighbors.push(point);
            }
        }
        // down
        if self.location.y < map.max_y {
            let point = Point { x: self.location.x, y: self.location.y + 1};
            let other = map.contents.get(&point).unwrap();

            if ! walls.contains(&point) && other.used <= self.size {
                neighbors.push(point);
            }
        }

        neighbors
    }
}

fn create_serverdata(file_name: &String) -> ServerMap {
    let mut servers = BTreeMap::new();
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let mut max_x = 0;
    let mut max_y = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 || ! line.starts_with('/') {
            continue;
        }
        let s = Server::new(&line);
        let p = s.location.clone();
        max_x = if p.x > max_x { p.x } else { max_x };
        max_y = if p.y > max_y { p.y } else { max_y };
        servers.insert(p, s);
    }
    {
        let max_p = Point { x: max_x, y: 0 };
        let s: &mut Server = servers.get_mut(&max_p).unwrap();
        s.contains_goal_data = true;
    }
    
    ServerMap {
        max_x: max_x,
        max_y: max_y,
        contents: servers,
    }
}

fn get_viable_pairs(servers: &BTreeMap<Point, Server>) -> Vec<(Server, Server)> {
    let mut pairs = Vec::new();
    let keys: Vec<&Point> = servers.keys().collect();
    for i in 0..keys.len() {
        let a = servers.get(keys[i]).unwrap();
        if a.used == 0 {
            continue;
        }
        for j in 0..keys.len() {
            if keys[i] != keys[j] {
                let b = servers.get(keys[j]).unwrap();
                if a.used <= b.avail {
                    pairs.push((a.clone(), b.clone()));
                }
            }
        }
    }

    pairs
}

fn draw_map(
    map: &ServerMap,
    movable: &HashSet<Server>,
    target: &HashSet<Server>,
    path: &Vec<Point>
)
{
    let max_x = map.max_x;
    let max_y = map.max_y;
    
    for y in 0..max_y+1 {
        if y == 0 {
            print!("(");
        } else {
            print!(" ");
        }
        for x in 0..max_x+1 {
            let key = Point { x: x, y: y };
            let server = map.contents.get(&key).unwrap();
            if path.contains(&key) {
                print!("o");
            } else if server.contains_goal_data {
                print!("G");
            } else if target.contains(&server) {
                print!("_");
            } else if movable.contains(&server) {
                print!(".");
            } else {
                print!("#");
            }
            if x == 0 && y == 0 {
                print!(")");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn get_shortest_path(
    start: &Point,
    end: &Point,
    map: &ServerMap,
    walls: &HashSet<Point>) -> Vec<Point>
{

    let mut came_from: BTreeMap<Point, Option<Point>> = BTreeMap::new();
    let mut frontier: Vec<Point> = Vec::new();
    let mut path: Vec<Point> = Vec::new();
    
    frontier.push(start.clone());
    came_from.insert(start.clone(), None);

    'outer: loop {
        if frontier.len() == 0 {
            println!("Frontier empty - cannot find path!");
            break 'outer;
        }
        let current: Vec<Point> = frontier.drain(0..).collect();
        for node in &current {
            // println!("At {:?}", node);
            if node == end {
                // println!("At goal {:?}", node);
                // Construct path by backtracking
                path.push(end.clone());
                let mut current = end.clone();
                while current != *start {
                    let c = came_from.get(&current);
                    match c {
                        Some(x) => {
                            current = x.clone().unwrap();
                            path.push(current.clone());
                        },
                        None => {
                            println!("Path incomlete!");
                            break;
                        }
                    }
                }
                break 'outer;
            }
            for neighbor in map.contents.get(&node).unwrap().neighbors(&map, &walls) {
                if ! came_from.contains_key(&neighbor) {
                    frontier.push(neighbor.clone());
                    came_from.insert(neighbor, Some(node.clone()));
                }
            }
        }
    }
    
    path
}

fn main() {
    println!("AOC 22");

    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let mut map = create_serverdata(&file_name);
    println!("Dimensions x/y: {}/{}, node count: {}",
             map.max_x, map.max_y, map.contents.len());
    
    let pairs = get_viable_pairs(&map.contents);

    println!("Pairs: {}", pairs.len());

    let mut src_set: HashSet<Server> = HashSet::new();
    let mut tgt_set: HashSet<Server> = HashSet::new();
    let mut unmovable: HashSet<Point> = HashSet::new();

    for p in pairs {
        src_set.insert(p.0.clone());
        tgt_set.insert(p.1.clone());
    }
    for v in map.contents.values() {
        if ! src_set.contains(v) && ! tgt_set.contains(v) {
            unmovable.insert(v.location.clone());
        }
    }
    
    println!("Starting points: {}", tgt_set.len());
    for s in &tgt_set {
        println!("{:?}", s.location);
    }
    // println!("Walls: {} \n {:?}", unmovable.len(), unmovable);

    let starts: Vec<Server> = tgt_set.iter().cloned().collect();
    let mut full_path: Vec<Point> = Vec::new();

    if starts.len() == 1 {
        let mut steps: usize = 0;
        let mut start = starts[0].clone();
        let access_node = Point { x: 0, y: 0 };
        let mut data_node = Point { x: map.max_x, y: 0 };
        
        while data_node != access_node {
            let goal = Point { x: data_node.x - 1, y: 0 };
            unmovable.insert(data_node.clone());
            
            let path = get_shortest_path(
                &start.location, &goal, &map, &unmovable);
            if path.len() == 0 {
                break;
            }
            full_path.extend(path.iter().cloned());
            steps += path.len();

            unmovable.remove(&data_node);

            if start == starts[0] {
                println!("Initial path");
                draw_map(&map, &src_set, &tgt_set, &path);
            }

            // Swap goal forward
            start.location.x = data_node.x;
            start.location.y = data_node.y;
            let mut g = map.contents.get_mut(&data_node).unwrap();
            g.contains_goal_data = false;
            
            data_node.x = data_node.x - 1;
        }
        println!("Total steps: {}", steps);
        
    }
    println!("Final path");
    draw_map(&map, &src_set, &tgt_set, &full_path);
    
    
}
