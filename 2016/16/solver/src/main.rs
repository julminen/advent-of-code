use std::env;

// part 1
// input = 10001110011110000
// disk length = 272

fn modified_dragon_curve(input: &Vec<u8>) -> Vec<u8> {
    let mut curve: Vec<u8> = Vec::with_capacity(input.len() * 2 + 1);
    let mut b: Vec<u8> = Vec::with_capacity(input.len());
    for f in 0..input.len() {
        let r = input.len() - f - 1;
        b.push(
            match input[r] {
                1 => 0,
                0 => 1,
                _ => panic!("Bad bit: {}", input[r]),
            }
            );
        curve.push(input[f]);
    }
    curve.push(0);
    for e in b {
        curve.push(e);
    }
    curve
}

fn calculate_checksum(bits: &Vec<u8>) -> Vec<u8> {
    let mut checksum: Vec<u8> = Vec::with_capacity(bits.len() / 2);

    for b in (0..bits.len()).filter(|&x| x % 2 == 1) {
        checksum.push(
            if bits[b-1] == bits[b] {
                1
            } else {
                0
            }
            );
        
    }
    if checksum.len() % 2 == 0 {
        calculate_checksum(&checksum)
    } else {
        checksum
    }
}

fn main() {
    println!("AOC 16");

    let input: Vec<char>;
    let disk_size: usize;
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        input = args[1].clone().chars().collect();
        disk_size = args[2].parse().unwrap();
    } else {
        println!("Usage: {} input disk_length", args[0]);
        return;
    }

    let mut fill: Vec<u8> = Vec::with_capacity(input.len());
    for e in input {
        fill.push(
            match e {
                '1' => 1,
                '0' => 0,
                _ => panic!("Bad input: {}", e),
            }
            );
    }

    println!("Original ({}): {:?}", fill.len(), fill);
    while fill.len() < disk_size {
        fill = modified_dragon_curve(&fill);
    }
    fill.truncate(disk_size);

    let checksum = calculate_checksum(&fill);
    print!("Checksum: ");
    for e in checksum {
        print!("{}", e);
    }
    println!("");
    
}
