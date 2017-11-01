//extern crate bmp;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp;

use common;
use common::AocResultType;
use common::AocResult;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const SIM_STEPS: usize = 100;

struct LedMatrix {
    matrix: [[[bool; WIDTH]; HEIGHT]; 2],
    active: usize,
    perfect: bool,
}


impl LedMatrix {
    fn load(file_name: &str, perfect: bool) -> LedMatrix {
        let reader = BufReader::new(File::open(file_name).unwrap());
        let mut row = 0;
        let mut col = 0;
        let mut lm = LedMatrix {
            matrix: [[[false; WIDTH]; HEIGHT]; 2],
            active: 0,
            perfect: perfect,
        };
        'outer: for line in reader.lines() {
            let line = line.unwrap();
            for c in line.chars() {
                match c {
                    '#' => lm.matrix[0][col][row] = true,
                    '.' => {}
                    _ => panic!("Bad char in spec: {}", c),
                }
                col += 1;
                if col == WIDTH {
                    col = 0;
                    row += 1;
                }
                if col > HEIGHT {
                    println!("WARNING: Too long spec!");
                    break 'outer;
                }
            }
        }
        if !perfect {
            lm.matrix[0][0][0] = true;
            lm.matrix[0][0][WIDTH - 1] = true;
            lm.matrix[0][HEIGHT - 1][0] = true;
            lm.matrix[0][HEIGHT - 1][WIDTH - 1] = true;
        }
        lm
    }
    fn neighbors_on(&self, x: usize, y: usize) -> usize {
        let mut c = 0;
        let sy = if y == 0 { 0 } else { y - 1 };
        let ey = cmp::min(HEIGHT - 1, y + 1);
        let sx = if x == 0 { 0 } else { x - 1 };
        let ex = cmp::min(WIDTH - 1, x + 1);

        for cy in sy..ey + 1 {
            for cx in sx..ex + 1 {
                if cy != y || cx != x {
                    if self.matrix[self.active][cy][cx] {
                        c += 1;
                    }
                }
            }
        }

        c
    }

    fn next(&mut self) {
        let new_active = match self.active {
            0 => 1,
            1 => 0,
            _ => panic!("Bad active matrix: {}", self.active),
        };
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors_on = self.neighbors_on(x, y);
                let is_corner = (y == 0 || y == HEIGHT - 1) && (x == 0 || x == WIDTH - 1);

                self.matrix[new_active][y][x] = if !self.perfect && is_corner {
                    true
                } else {
                    if self.matrix[self.active][y][x] {
                        neighbors_on == 2 || neighbors_on == 3
                    } else {
                        neighbors_on == 3
                    }
                };
            }
        }
        self.active = new_active;
    }
    fn count_on(&self) -> usize {
        let mut on_count = 0;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.matrix[self.active][y][x] {
                    on_count += 1;
                }
            }
        }
        on_count
    }
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/18/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }

    let mut matrix = LedMatrix::load(&full_name, true);
    for _ in 0..SIM_STEPS {
        matrix.next();
    }
    let phase_1_on = matrix.count_on();

    let mut matrix = LedMatrix::load(&full_name, false);
    for _ in 0..SIM_STEPS {
        matrix.next();
    }
    let phase_2_on = matrix.count_on();

    let res: AocResult = AocResult {
        day: 18,
        phase_1: Some(AocResultType::Usize(phase_1_on)),
        phase_2: Some(AocResultType::Usize(phase_2_on)),
    };

    res
}
