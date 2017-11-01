use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;


#[derive(Debug)]
struct Tile {
    is_trap: bool,
    is_wall: bool,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '#' => Tile { is_trap: false, is_wall: true },
            '.' => Tile { is_trap: false, is_wall: false },
            '^' => Tile { is_trap: true, is_wall: false },
            _ => panic!("Bad tile: {}", c),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",
               if self.is_wall {
                   '#'
               } else {
                   if self.is_trap {
                       '^'
                   } else {
                       '.'
                   }
               }
        )
    }
}

fn next_row(row: &Vec<Tile>) -> Vec<Tile> {
    let mut nr:Vec<Tile> = Vec::with_capacity(row.len());
    nr.push(Tile::new('#'));

    for i in 1..row.len()-1 {
        let left = &row[i-1];
        let center = &row[i];
        let right = &row[i+1];

        nr.push(Tile::new(
            if left.is_trap && center.is_trap && ! right.is_trap {
                '^'
            } else if ! left.is_trap && center.is_trap && right.is_trap {
                '^'
            } else if left.is_trap && ! center.is_trap && ! right.is_trap {
                '^'
            } else if ! left.is_trap && ! center.is_trap && right.is_trap {
                '^'
            } else {
                '.'
            })
        );
                
        
    }
    
    nr.push(Tile::new('#'));

    nr
}

fn main() {
    println!("AOC 18");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    let mut map: Vec<Vec<Tile>> = Vec::new();

    println!("Reading {}", file_name);
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }

        let mut row: Vec<Tile> = Vec::with_capacity(line.len() + 2);
        row.push(Tile::new('#'));
        for c in line.chars() {
            row.push(Tile::new(c));
        }
        row.push(Tile::new('#'));

        map.push(row);
    }

    for i in 0..(400000-1) {
        let next = next_row(&map[i]);
        map.push(next);
    }

    let mut spaces: usize = 0;
    for row in 0..map.len() {
        //print!("{:02} : ", row+1);
        for col in &map[row] {
            //print!("{}", col);
            if ! col.is_wall && ! col.is_trap {
                spaces += 1;
            }
        }
        //println!("");
    }

    println!("Spaces: {}", spaces);
    
}
