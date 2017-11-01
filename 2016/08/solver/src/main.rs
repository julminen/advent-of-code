use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;


struct Display {
    matrix: [[char; 50]; 6],
    x_ind: [usize; 6],
}

impl Display {
    pub fn new() -> Display {
        Display {
            matrix: [[' '; 50]; 6],
            x_ind: [0; 6],
        }
    }
    pub fn rotate_row(&mut self, row: usize, amount: usize) {
        let row_len = self.matrix[0].len();
        let roll = amount % row_len;
        self.x_ind[row] = self.x_ind[row] + row_len - roll;
    }
    pub fn rotate_column(&mut self, column: usize, amount: usize) {
        let rows = self.matrix.len();
        let columns = self.matrix[0].len();
        let roll = amount % rows;

        for _ in 0..roll {
            let tmp = self.matrix[rows-1][(self.x_ind[rows-1] + column) % columns];
            for i in (1..rows).rev() {
                self.matrix[i][(self.x_ind[i] + column) % columns] = self.matrix[i-1][(self.x_ind[i-1] + column) % columns];
            }
            self.matrix[0][(self.x_ind[0] + column) % columns] = tmp;
        }
    }
    pub fn rect(&mut self, columns: usize, rows: usize) {
        let row_length = self.matrix[0].len();
        for y in 0..rows {
            for x in 0..columns {
                let x = (self.x_ind[y] + x) % row_length;
                self.matrix[y][x] = '#';
            }
        }
    }
    pub fn voltage(&self) -> u32 {
        let mut voltage = 0;
        let rows = self.matrix.len();
        let columns = self.matrix[0].len();
        
        for y in 0..rows {
            for x in 0..columns {
                if self.matrix[y][x] == '#' {
                    voltage += 1;
                }
            }
        }
        return voltage;
    }
    pub fn print(&self) {
        for _ in 0..self.matrix[0].len()*2 + 2 {
            print!("-");
        }
        println!("");
        for y in 0..self.matrix.len() {
            print!("|");
            for x in 0..self.matrix[y].len() {
                let dx = (x + self.x_ind[y]) % self.matrix[y].len();
                print!("{} ", self.matrix[y][dx as usize]);
            }
            println!("|");
        }
        for _ in 0..self.matrix[0].len()*2 + 2 {
            print!("-");
        }
        println!("");
        println!("Voltage: {}", self.voltage());
    }
}


fn main() {

    println!("AOC 08");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };
    let mut lcd = Display::new();

    println!("Reading {}", file_name);
    
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let cmd: Vec<&str> = line.split(' ').collect();
        match cmd[0] {
            "rect" => {
                let dims: Vec<&str> = cmd[1].split('x').collect();
                let x: usize = dims[0].parse().unwrap();
                let y: usize = dims[1].parse().unwrap();
                lcd.rect(x, y);
            },
            "rotate" => {
                match cmd[1] {
                    "column" => {
                        let x: Vec<&str> = cmd[2].split('=').collect();
                        let col: usize = x[1].parse().unwrap();
                        let amount: usize = cmd[4].parse().unwrap();
                        lcd.rotate_column(col, amount);
                    },
                    "row" => {
                        let y: Vec<&str> = cmd[2].split('=').collect();
                        let row: usize = y[1].parse().unwrap();
                        let amount: usize = cmd[4].parse().unwrap();
                        lcd.rotate_row(row, amount);
                    },
                    _ => println!("Bad rotate command!"),
                }
            },
            _ => println!("unknown command!"),
        };
        lcd.print();
    }
}
