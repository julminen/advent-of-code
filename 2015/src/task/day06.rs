extern crate bmp;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use common;
use common::AocResultType;
use common::AocResult;

// TODO: dynamically allocate memory, this causes stack overflow otherwise
struct LedMatrix {
    // matrix: [[bool; 1000]; 1000],
    matrix: Vec<Vec<bool>>,
    // tuned_matrix: [[u8; 1000]; 1000],
    tuned_matrix: Vec<Vec<u8>>,
}

impl LedMatrix {
    fn new(width: usize, height: usize) -> LedMatrix {
        let matrix: Vec<Vec<bool>> = vec![vec![false; width]; height];
        let tuned_matrix: Vec<Vec<u8>> = vec![vec![0; width]; height];
        LedMatrix {
            matrix,
            tuned_matrix,
        }
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl LedMatrix {
    fn corner_points(p1: &Point, p2: &Point) -> (Point, Point) {
        // find left upper point
        let a = Point {
            x: if p1.x < p2.x { p1.x } else { p2.x },
            y: if p1.y < p2.y { p1.y } else { p2.y },
        };
        // find right lower point
        let b = Point {
            x: if p1.x > p2.x { p1.x } else { p2.x },
            y: if p1.y > p2.y { p1.y } else { p2.y },
        };
        (a, b)
    }

    fn on(&mut self, p1: &Point, p2: &Point) {
        let (a, b) = LedMatrix::corner_points(p1, p2);
        for y in a.y..b.y + 1 {
            for x in a.x..b.x + 1 {
                self.matrix[y][x] = true;
                self.tuned_matrix[y][x] += 1;
            }
        }
    }
    fn off(&mut self, p1: &Point, p2: &Point) {
        let (a, b) = LedMatrix::corner_points(p1, p2);
        for y in a.y..b.y + 1 {
            for x in a.x..b.x + 1 {
                self.matrix[y][x] = false;
                if self.tuned_matrix[y][x] > 0 {
                    self.tuned_matrix[y][x] -= 1;
                }
            }
        }
    }
    fn toggle(&mut self, p1: &Point, p2: &Point) {
        let (a, b) = LedMatrix::corner_points(p1, p2);
        for y in a.y..b.y + 1 {
            for x in a.x..b.x + 1 {
                self.matrix[y][x] = !self.matrix[y][x];
                self.tuned_matrix[y][x] += 2;
            }
        }
    }
    fn count_on(&self) -> usize {
        let mut on_count = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                if self.matrix[y][x] {
                    on_count += 1;
                }
            }
        }
        on_count
    }
    fn count_brightness(&self) -> usize {
        let mut brightness = 0;
        for y in 0..self.matrix.len() {
            for x in 0..self.matrix[0].len() {
                brightness += self.tuned_matrix[y][x] as usize;
            }
        }
        brightness
    }
    fn store_tuned_image(&self, file_name: &str) {
        let ydim = self.tuned_matrix.len();
        let xdim = self.tuned_matrix[0].len();
        let mut img = bmp::Image::new(xdim as u32, ydim as u32);
        for y in 0..ydim {
            for x in 0..xdim {
                let c = self.tuned_matrix[y][x];
                img.set_pixel(x as u32, y as u32, bmp::Pixel::new(c, c, c));
            }
        }
        img.save(file_name).unwrap();
    }
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/06/");
    let mut generate_bmp = false;
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
            let gen_images = common::get_input("Generate images?", "no");
            match gen_images.as_str() {
                "yes" => generate_bmp = true,
                _ => {
                    generate_bmp = false;
                    println!("No bmps");
                }
            }
        }
    }

    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 6,
        phase_1: None,
        phase_2: None,
    };

    let mut leds = LedMatrix::new(1000, 1000);

    for (l, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let cmd: Vec<&str> = line.split_whitespace().collect();
        match cmd[0] {
            "turn" => {
                let coords_1: Vec<&str> = cmd[2].split(',').collect();
                let coords_2: Vec<&str> = cmd[4].split(',').collect();
                let p1 = Point {
                    x: coords_1[0].parse().unwrap(),
                    y: coords_1[1].parse().unwrap(),
                };
                let p2 = Point {
                    x: coords_2[0].parse().unwrap(),
                    y: coords_2[1].parse().unwrap(),
                };
                match cmd[1] {
                    "on" => {
                        leds.on(&p1, &p2);
                    }
                    "off" => {
                        leds.off(&p1, &p2);
                    }
                    _ => panic!("Bad command: {}", line),
                }
            }
            "toggle" => {
                let coords_1: Vec<&str> = cmd[1].split(',').collect();
                let coords_2: Vec<&str> = cmd[3].split(',').collect();
                let p1 = Point {
                    x: coords_1[0].parse().unwrap(),
                    y: coords_1[1].parse().unwrap(),
                };
                let p2 = Point {
                    x: coords_2[0].parse().unwrap(),
                    y: coords_2[1].parse().unwrap(),
                };
                leds.toggle(&p1, &p2);
            }
            _ => panic!("Unknown command: '{}'", line),
        }
        if generate_bmp {
            let img_name = format!("input/06/output/img_{:03}.bmp", l);
            leds.store_tuned_image(&img_name);
        }
    }
    res.phase_1 = Some(AocResultType::Usize(leds.count_on()));
    res.phase_2 = Some(AocResultType::Usize(leds.count_brightness()));
    res
}
