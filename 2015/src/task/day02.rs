use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use common;
use common::AocResultType;
use common::AocResult;

#[derive(Debug)]
struct Box {
    l: usize,
    w: usize,
    h: usize,
    paper: usize,
    ribbon: usize,
}

impl Box {
    fn new(spec: &str) -> Box {
        let dims: Vec<&str> = spec.split('x').collect();
        let l = dims[0].parse().unwrap();
        let w = dims[1].parse().unwrap();
        let h = dims[2].parse().unwrap();
        Box {
            l: l,
            w: w,
            h: h,
            paper: Box::required_paper(l, w, h),
            ribbon: Box::required_ribbon(l, w, h),
        }
    }

    fn required_paper(l: usize, w: usize, h: usize) -> usize {
        // 2*l*w + 2*w*h + 2*h*l + smallest side
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        let smallest = if lw <= wh && lw <= hl {
            lw
        } else if wh <= lw && wh <= hl {
            wh
        } else {
            hl
        };
        2 * lw + 2 * wh + 2 * hl + smallest
    }

    fn required_ribbon(l: usize, w: usize, h: usize) -> usize {
        // shortest distance around sides + vol of box
        let mut sides: Vec<usize> = Vec::with_capacity(3);
        sides.push(l);
        sides.push(w);
        sides.push(h);
        sides.sort();
        let shortest_dist = sides[0] * 2 + sides[1] * 2;

        // Add bow
        shortest_dist + sides[0] * sides[1] * sides[2]
    }
}

pub fn solve(file_name: Option<&str>) -> AocResult {
    let mut full_name = String::from("input/02/");
    match file_name {
        Some(name) => full_name.push_str(name),
        None => {
            let name = common::get_input("File name", "input");
            full_name.push_str(&name);
        }
    }
    let reader = BufReader::new(File::open(full_name).unwrap());

    let mut res: AocResult = AocResult {
        day: 2,
        phase_1: None,
        phase_2: None,
    };

    let mut boxes: Vec<Box> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        boxes.push(Box::new(&line.as_str()));
    }

    let mut total_area = 0;
    let mut total_ribbon = 0;
    for b in &boxes {
        total_area += b.paper;
        total_ribbon += b.ribbon;
    }
    res.phase_1 = Some(AocResultType::Usize(total_area));
    res.phase_2 = Some(AocResultType::Usize(total_ribbon));

    res
}
