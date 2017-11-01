use std::collections::BTreeMap;
use std::cmp::Ordering;

const INPUT: usize = 1362;
// const INPUT: usize = 10;


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x == other.x && self.y == other.y {
            Ordering::Equal
        } else {
            if self.x < other.x {
                Ordering::Less
            } else if self.x > other.x {
                Ordering::Greater
            } else {
                if self.y < other.y {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        
        if self.x > 0 && is_space(self.x-1, self.y) {
            neighbors.push(Point{x: self.x-1, y: self.y});
        }
        if is_space(self.x+1, self.y) {
            neighbors.push(Point{x: self.x+1, y: self.y});
        }
        if self.y > 0 && is_space(self.x, self.y-1) {
            neighbors.push(Point{x: self.x, y: self.y-1});
        }
        if is_space(self.x, self.y+1) {
            neighbors.push(Point{x: self.x, y: self.y+1});
        }
        
        // println!("Point {:?} neighbors: {:?}", self, neighbors);

        neighbors
    }
}

fn find_path(start: Point, goal: Point) -> Vec<Point> {
    let mut frontier: Vec<Point> = Vec::new();
    let mut came_from: BTreeMap<Point, Option<Point>> = BTreeMap::new();
    frontier.push(start);
    came_from.insert(start, None);
    'outer: loop {
        if frontier.len() == 0 {
            println!("Frontier empty, stopping");
            println!("Came from: {:?}", came_from);
            break 'outer;
        }
        let hp: Vec<Point> = frontier.drain(0..).collect();
        for p in hp {
            // println!("At {:?}", p);
            if p == goal {
                println!("Reached goal!");
                break 'outer;
            }
            for n in p.neighbors() {
                if ! came_from.contains_key(&n) {
                    frontier.push(n);
                    came_from.insert(n, Some(p));
                }
                
            }
        }
    }

    let mut path: Vec<Point> = Vec::new();
    path.push(goal);
    let mut current = goal;
    while current != start {
        let c = came_from.get(&current);
        match c {
            Some(x) => {
                current = x.unwrap();
                path.push(current);
            },
            None => {
                println!("Could not reach end!");
                break;
            }
        }
    }

    path
}

fn reachable(start: Point, max_steps: usize) -> usize {
    let mut frontier: Vec<Point> = Vec::new();
    let mut came_from: BTreeMap<Point, Option<Point>> = BTreeMap::new();
    let mut step = 0;
    frontier.push(start);
    came_from.insert(start, None);
    'outer: loop {
        if frontier.len() == 0 {
            println!("Frontier empty, stopping");
            println!("Came from: {:?}", came_from);
            break 'outer;
        }
        if step == max_steps {
            println!("Took {} steps", step);
            break 'outer;
        }
        step += 1;
        let hp: Vec<Point> = frontier.drain(0..).collect();
        for p in hp {
            // println!("At {:?}", p);
            for n in p.neighbors() {
                if ! came_from.contains_key(&n) {
                    frontier.push(n);
                    came_from.insert(n, Some(p));
                }
                
            }
        }
    }

    came_from.len()

}

fn is_space(x: usize, y: usize) -> bool {
    let w = x*x + 3*x + 2*x*y + y + y*y + INPUT;
    w.count_ones() % 2 == 0
}

fn draw_maze(width: usize, height: usize, start: Point, target: Point, path: Vec<Point>) {
    print!("   ");
    for x in 0..width {
        print!("{:}", x%10);
    }
    println!("");
    for y in 0..height {
        print!("{:2} ", y);
        for x in 0..width {
            let p = Point{x: x, y: y};
            if is_space(x, y) {
                if start.x == x && start.y == y {
                    print!("H");
                } else if target.x == x && target.y == y {
                    print!("X");
                } else if path.contains(&p) {
                    print!("o");
                } else {
                    print!(" ")
                }
            } else {
                print!("█");
            }
        }
        println!("");
    }
}

fn main() {
    println!("AOC 13");
    
    let me = Point{x: 1, y:1};
    let target = Point{x: 31, y: 39};
    // let target = Point{x: 7, y: 4};

    let path = find_path(me, target);
    println!("Len {}, steps = {}", path.len(), path.len()-1);

    draw_maze(70, 45, me, target, path);

    println!("Reachable in 50 steps: {}", reachable(me, 50));

}
