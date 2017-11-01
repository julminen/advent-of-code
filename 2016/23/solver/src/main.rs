use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
enum OpType {
    Cpy,
    Inc,
    Dec,
    Jnz,
    Tgl,
}

#[derive(Debug)]
struct Memory {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Memory {
    fn new() -> Memory {
        Memory {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    }
    fn get(&self, s: &str) -> i32 {
        match s {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            _ => s.parse().unwrap(),
        }
    }
    fn set(&mut self, s: &str, val: i32) {
        match s {
            "a" => self.a = val,
            "b" => self.b = val,
            "c" => self.c = val,
            "d" => self.d = val,
            _ => panic!("Bad register set: {} to {}", s, val),
        }
    }
    fn incr(&mut self, s: &str) {
        match s {
            "a" => self.a += 1,
            "b" => self.b += 1,
            "c" => self.c += 1,
            "d" => self.d += 1,
            _ => panic!("Bad register incr: {}", s),
        }
    }
    fn decr(&mut self, s: &str) {
        match s {
            "a" => self.a -= 1,
            "b" => self.b -= 1,
            "c" => self.c -= 1,
            "d" => self.d -= 1,
            _ => panic!("Bad register decr: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
struct OpCode {
    op_type: OpType,
    arguments: Vec<String>,
}

impl OpCode {
    fn new(type_str: &str, arg1: &str, arg2: &str) -> OpCode {
        let op_type = 
            match type_str {
                "cpy" => OpType::Cpy,
                "inc" => OpType::Inc,
                "dec" => OpType::Dec,
                "jnz" => OpType::Jnz,
                "tgl" => OpType::Tgl,
                _ => panic!("Bad opcode: {}", type_str),
            };
        let mut args = Vec::new();
        args.push(arg1.to_string());
        match op_type {
            OpType::Cpy | OpType::Jnz | OpType::Tgl => {
                args.push(arg2.to_string())
            },
            _ => {},
        }
        OpCode {
            op_type: op_type,
            arguments: args,
        }
    }

    fn execute(
        &mut self,
        mem: &mut Memory,
        pc: usize,
        code: &mut Vec<OpCode>
    ) -> usize
    {
        match self.op_type {
            OpType::Cpy => {
                let src = mem.get(self.arguments[0].as_str());
                match self.arguments[1].parse::<i32>() {
                    Err(_) => mem.set(self.arguments[1].as_str(), src),
                    Ok(_) => {}, // skip
                };
                pc + 1
            },
            OpType::Inc => {
                mem.incr(self.arguments[0].as_str());
                pc + 1
            },
            OpType::Dec => {
                mem.decr(self.arguments[0].as_str());
                pc + 1
            },
            OpType::Jnz => {
                let cmp = mem.get(self.arguments[0].as_str());
                let jmp = mem.get(self.arguments[1].as_str());
                if cmp != 0 {
                    (pc as isize + jmp as isize) as usize
                } else {
                    pc + 1
                }
            },
            OpType::Tgl => {
                let tgt = (pc as i32) + mem.get(self.arguments[0].as_str());
                if tgt >= 0 && (tgt as usize) < code.len() {
                    code[tgt as usize].toggle();
                }
                pc + 1
            },
        }
    }
    fn toggle(&mut self) {
        match self.op_type {
            // One arg
            OpType::Inc => self.op_type = OpType::Dec,
            OpType::Dec => self.op_type = OpType::Inc,
            OpType::Tgl => self.op_type = OpType::Inc,
            // Two arg
            OpType::Jnz => self.op_type = OpType::Cpy,
            OpType::Cpy => self.op_type = OpType::Jnz,
        }
    }
}

fn read_code(file_name: &String) -> Vec<OpCode> {
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    let mut code = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{:02}: {}", i, line);
        if line.len() == 0 {
            continue;
        }
        let cmd: Vec<_> = line.split_whitespace().collect();
        code.push(
            OpCode::new(
                cmd[0],
                if cmd.len() > 1 { cmd[1] } else { "" },
                if cmd.len() > 2 { cmd[2] } else { "" }
                ));
    }
    code
}

fn try_optimize(pc: usize, code: &Vec<OpCode>, mem: &mut Memory) -> usize {
    // Check for multiply
    if pc + 5 < code.len() &&
        code[pc+0].op_type == OpType::Cpy &&
        code[pc+1].op_type == OpType::Inc &&
        code[pc+2].op_type == OpType::Dec &&
        code[pc+3].op_type == OpType::Jnz &&
        code[pc+4].op_type == OpType::Dec &&
        code[pc+5].op_type == OpType::Jnz
    {
        let target = code[pc+1].arguments[0].as_str();
        let inner_val = code[pc+0].arguments[0].as_str();
        let outer = code[pc+4].arguments[0].as_str();
        let inner_var = code[pc+2].arguments[0].as_str();
        let res = mem.get(target) + mem.get(inner_val) * mem.get(outer);
        mem.set(target, res);
        mem.set(inner_var, 0);
        mem.set(outer, 0);
        pc + 5
    } else {
        pc
    }
}

fn main() {
    println!("AOC 23");

    let args: Vec<String> = env::args().collect();
    let eggs: i32 = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        0
    };
    
    let file_name = if args.len() > 2 {
        args[2].clone()
    } else {
        "example".to_string()
    };

    println!("Input file: {}, eggs: {}", file_name, eggs);

    let mut code = read_code(&file_name);
    let mut mem = Memory::new();
    mem.a = eggs;

    let breakpoints: Vec<usize> = vec![100];
    let mut pc = 0;
    let now = Instant::now();
    
    loop {
        if breakpoints.contains(&pc) {
            let dur = now.elapsed();
            println!("[{:03}.{:09}] At {:02}: {:?}",
                     dur.as_secs(), dur.subsec_nanos(), pc, mem);
        }
        pc = try_optimize(pc, &code, &mut mem);
        if pc >= code.len() {
            break;
        }
        let mut op = code[pc].clone();
        pc = op.execute(&mut mem, pc, &mut code);
    }
    println!("Execution finished\nMemory:\na: {}\nb: {}\nc: {}\nd: {}",
             mem.a, mem.b, mem.c, mem.d);
}
