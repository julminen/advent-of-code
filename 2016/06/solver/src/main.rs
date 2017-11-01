use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeMap;

fn calc(reader: &mut BufReader<File>) {

    // Frequencies
    let mut freqs: Vec<BTreeMap<char, i32>> = Vec::with_capacity(8);

    // Read first line to know line length
    let mut buf = String::with_capacity(20);
    reader.read_line(&mut buf).unwrap();
    // Create treemaps and insert first values
    for (index, c) in buf.trim().chars().enumerate() {
        freqs.push(BTreeMap::new());
        freqs[index].insert(c, 1);
    }
    // Read the rest of file
    for l in reader.lines() {
        let line = l.unwrap();
        for (index, c) in line.trim().chars().enumerate() {
            if freqs[index].contains_key(&c) {
                let mut v = freqs[index].get_mut(&c).unwrap();
                *v += 1;
            } else {
                freqs[index].insert(c, 1);
            }
        }
    }

    // Most common letters
    let mut answer_1: Vec<char> = Vec::with_capacity(freqs.len());
    // Least common letters
    let mut answer_2: Vec<char> = Vec::with_capacity(freqs.len());

    // Find most and least common letters
    for map in freqs.iter() {
        let mut max = 0;
        let mut min = 99999;
        let mut max_chr = ' ';
        let mut min_chr = ' ';
        for (key, value) in map.iter() {
            if *value > max {
                max_chr = *key;
                max = *value;
            }
            if *value < min {
                min_chr = *key;
                min = *value;
            }
        }
        answer_1.push(max_chr);
        answer_2.push(min_chr);
    }
    println!("---");
    for c in answer_1.iter() {
        print!("{}", c);
    }
    println!("");
    for c in answer_2.iter() {
        print!("{}", c);
    }
    println!("");
}


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f);
    calc(&mut reader);

}
