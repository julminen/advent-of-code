use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

fn main() {
    println!("AOC 10");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    let debug = file_name == "example";
    
    println!("Reading {}", file_name);
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    let mut pc = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;

    let step_limit = 10000;
    let mut steps = 0;

    loop {
        if pc >= lines.len() {
            println!("\nHALT: a: {}, b: {}, c: {}, d: {}, steps: {}", a, b, c, d, steps);
            break;
        }
        steps += 1;
        if debug && steps > step_limit {
            println!("BREAK");
            break;
        }
        if debug {
            print!("{:12} : pc {:2} : ", lines[pc], pc);
        }
        let cmd: Vec<_> = lines[pc].split(' ').collect();
        match cmd[0] {
            "cpy" => {
                let src = match cmd[1] {
                    "a" => a,
                    "b" => b,
                    "c" => c,
                    "d" => d,
                    _ => cmd[1].parse().unwrap(),
                };
                match cmd[2] {
                    "a" => a = src,
                    "b" => b = src,
                    "c" => c = src,
                    "d" => d = src,
                    _ => panic!("Bad destination: {}", cmd[2]),
                };
                if debug {
                    println!("{} -> {}", src, cmd[2]);
                }
                pc += 1;
            },
            "inc" => {
                let val = match cmd[1] {
                    "a" => {a += 1; a},
                    "b" => {b += 1; b},
                    "c" => {c += 1; c},
                    "d" => {d += 1; d},
                    _ => panic!("Bad increment: {}", cmd[1]),
                };
                if debug {
                    println!("{}++ -> {}", cmd[1], val);
                }
                pc += 1;
            },
            "dec" => {
                let val = match cmd[1] {
                    "a" => {a -= 1; a},
                    "b" => {b -= 1; b},
                    "c" => {c -= 1; c},
                    "d" => {d -= 1; d},
                    _ => panic!("Bad decrement: {}", cmd[1]),
                };
                if debug {
                    println!("{}-- -> {}", cmd[1], val);
                }
                pc += 1;
            },
            "jnz" => {
                let cmp = match cmd[1] {
                    "a" => a,
                    "b" => b,
                    "c" => c,
                    "d" => d,
                    _ => cmd[1].parse().unwrap(),
                };
                if cmp != 0 {
                    let jmp: isize = cmd[2].parse().unwrap();
                    pc = (pc as isize + jmp) as usize;
                } else {
                    pc += 1;
                }
                if debug {
                    println!("{} -> PC = {}",  cmp, pc);
                }
            },
            _ => {
                println!("Unknown command {}!", cmd[0]);
                panic!("BAD OPCODE");
            }
        }
    }
    
}
