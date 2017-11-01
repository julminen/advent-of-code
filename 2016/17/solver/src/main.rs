extern crate md5;

use std::env;

const MAX_Y: usize = 3;
const MAX_X: usize = 3;

#[derive (Debug)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, Clone)]
struct Room {
    x: usize,
    y: usize,
}

impl Room {
    fn next_room(&self, dir: &Direction) -> Room {
        let x = match *dir {
            Direction::Left => {
                if self.x == 0 {
                    panic!("Hit left wall!");
                }
                self.x - 1
            },
            Direction::Right => {
                if self.x == MAX_X {
                    panic!("Hit right wall!");
                }
                self.x + 1
            },
            _ => self.x,
        };
        let y = match *dir {
            Direction::Up => {
                if self.y == 0 {
                    panic!("Hit top wall!");
                }
                self.y - 1
            },
            Direction::Down => {
                if self.y == MAX_Y {
                    panic!("Hit bottom wall!");
                }
                self.y + 1
            },
            _ => self.y,
        };
        Room {x: x, y: y}
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    path: Vec<u8>,
    room: Room,
    length: usize,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let p = String::from_utf8(self.path.clone()).unwrap();
        write!(f, "Path to room ({}, {}), len {} = {}",
               self.room.x,
               self.room.y,
               self.length,
               p)
    }
}

fn get_directions(room: &Room, key: &Vec<u8>) -> Vec<Direction> {
    let mut dirs: Vec<Direction> = Vec::with_capacity(4);
    let hash = md5::compute(key.as_slice());

    match hash[0] >> 4 {
        0xb...0xf => {
            if room.y > 0 {
                dirs.push(Direction::Up);
            }
        }
        _ => {},
    }
    match hash[0] & 0x0f {
        0xb...0xf => {
            if room.y < MAX_Y { 
                dirs.push(Direction::Down);
            }
        }
        _ => {},
    }
    match hash[1] >> 4 {
        0xb...0xf => {
            if room.x > 0 {
                dirs.push(Direction::Left);
            }
        }
        _ => {},
    }
    match hash[1] & 0x0f {
        0xb...0xf => {
            if room.x < MAX_X { 
                dirs.push(Direction::Right);
            }
        }
        _ => {},
    }
    
    dirs
}

fn dir_to_u8(dir: &Direction) -> u8 {
    match *dir {
        Direction::Up => 'U' as u8,
        Direction::Down => 'D' as u8,
        Direction::Left => 'L' as u8,
        Direction::Right => 'R' as u8,
    }
}

fn main() {
    let default_input = "qzthpkfp";
    println!("AOC 17");
    let args: Vec<String> = env::args().collect();

    let input: String = 
        if args.len() > 1 {
            args[1].clone()
        } else {
            println!("No input given, using default");
            default_input.to_string()
        };

    println!("Input: {}", input);

    let start: Path = Path {
        path: Vec::from(input.as_bytes()),
        room: Room {x: 0, y: 0},
        length: 0,
    };

    let mut longest_path: Path = start.clone();
    let mut shortest_path: Path = start.clone();
    shortest_path.length = usize::max_value();
    let mut path_count: usize = 0;
    
    let mut frontier: Vec<Path> = Vec::new();
    frontier.push(start);

    'outer: loop {
        let current: Vec<Path> = frontier.drain(0..).collect();
        if current.len() == 0 {
            println!("No more paths to search!");
            break 'outer;
        }
        for node in current {
            // println!("At {}", node);
            let next_directions = get_directions(&node.room, &node.path);
            for dir in next_directions {
                let room = node.room.next_room(&dir);
                let mut path = node.path.clone();
                path.push(dir_to_u8(&dir));

                let new_node = Path {
                    path: path,
                    room: room,
                    length: node.length + 1,
                };

                if new_node.room.x == MAX_X && new_node.room.y == MAX_Y {
                    // println!("Finished: {}", new_node);
                    path_count += 1;
                    if new_node.length < shortest_path.length {
                        shortest_path = new_node.clone();
                    }
                    if new_node.length > longest_path.length {
                        longest_path = new_node.clone();
                    }
                } else {
                    frontier.push(new_node);
                }
            }
        }
    }

    println!("Shortest path: {}", shortest_path);
    println!("Longest path: {}", longest_path);
    println!("Paths found: {}", path_count);

}
