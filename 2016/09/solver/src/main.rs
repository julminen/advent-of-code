use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;


fn decompress(line: &str) -> String {
    let mut result = String::with_capacity(100000);
    let mut chars = line.chars();
    let mut chr = chars.next();
    while chr != None {
        if chr.unwrap() == '(' {
            // Discard the (
            chr = chars.next();
            let mut buf = String::with_capacity(12);
            while chr.unwrap() != ')' {
                buf.push(chr.unwrap());
                chr = chars.next();
            }
            // println!("buf: {}", &buf);
            let cmd: Vec<&str> = buf.split('x').collect();
            let length: usize = cmd[0].parse().unwrap();
            let multiplier: usize = cmd[1].parse().unwrap();
            // println!("len {}, mp {}", length, multiplier);
            let mut dec_buf = String::with_capacity(length);
            for _ in 0..length {
                chr = chars.next();
                dec_buf.push(chr.unwrap());
            }
            // println!("{}: {}", &buf, &dec_buf);
            for _ in 0..multiplier {
                result.push_str(&dec_buf);
            }
        } else {
            result.push(chr.unwrap());
        }
        chr = chars.next();
    }
    
    return result;
}

fn decompressed_length(ci: &mut std::str::Chars, read_amount: usize) -> usize {
    let mut size: usize = 0;

    let mut c: char;
    let mut read: usize = 0;
    while read < read_amount {
        //println!("{} / {}", read, read_amount);
        c = ci.next().unwrap();
        read += 1;
        if c == '(' {
            // Discard the (
            c = ci.next().unwrap();
            read += 1;
            let mut buf = String::with_capacity(12);
            while c != ')' {
                buf.push(c);
                c = ci.next().unwrap();
                read += 1;
            }

            let cmd: Vec<&str> = buf.split('x').collect();
            let length: usize = cmd[0].parse().unwrap();
            let multiplier: usize = cmd[1].parse().unwrap();

            size += multiplier * decompressed_length(ci, length);
            read += length;
            
        } else {
            size += 1;
        }
    }
    
    return size;
}

fn main() {
    println!("AOC 09");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    println!("Reading file '{}'", file_name);
    
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        //println!("{}", line);

        let decomp = decompress(&line);

        let mut ci = line.chars();
        let decomp_2_length = decompressed_length(&mut ci, line.len());
        println!("Length: {} -> {}, v2: {}",
                 line.len(), decomp.len(), decomp_2_length);
        // println!("{} -> {}\n", line, decomp);
    }
}
