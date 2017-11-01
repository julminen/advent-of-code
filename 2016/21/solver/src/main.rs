use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::iter::FromIterator;

fn swap_position(password: &mut Vec<char>, a: usize, b: usize) {
    password.swap(a, b);
}

fn swap_letter(password: &mut Vec<char>, a: char, b: char) {
    for c in password.iter_mut() {
        if *c == a {
            *c = b;
        } else if *c == b {
            *c = a;
        }
    }
}

enum Direction {
    Left,
    Right,
}

fn rotate(password: &mut Vec<char>, dir: Direction, steps: usize, rev: bool) {
    let dir = match dir {
        Direction::Left => if rev { Direction::Right } else { Direction::Left },
        Direction::Right => if rev { Direction::Left } else { Direction::Right },
    };
    
    let steps = steps % password.len();
    let cut_from = 
        match dir {
            Direction::Left => steps,
            Direction::Right => password.len() - steps
        };
    let mut tmp: Vec<char> = Vec::with_capacity(password.len());
    {
        let (a, b) = password.split_at_mut(cut_from);
        tmp.extend_from_slice(b);
        tmp.extend_from_slice(a);
    }
    password.clear();
    password.extend(tmp);
}

fn rotate_based(password: &mut Vec<char>, chr: char, rev: bool) {
    // TODO: rev
    let location = password.iter().position(|&x| x == chr).unwrap();

    let mut steps = location;
    if location > 3 {
        steps += 1;
    }
    steps += 1;

    if rev {
        if password.len() != 8 {
            println!("TODO");
            return;
        }
        //if location == 0 {
        //    panic!("Irreversible");
        //}

        /* hardcoded
        fbgdceah : len: 8
        
        index: steps  : new index
        |   0: 1      : 1
        |   1: 2      : 3
        |   2: 3      : 5
        |   3: 4      : 7
        |   4: 6      : 10 (2)
        |   5: 7      : 12 (4)
        |   6: 8 (0)  : 14 (6)
        |   7: 9 (1)  : 16 (8) (0)
         */
        
        let new_location = 
            match location {
                0 => 7,
                1 => 0,
                2 => 4,
                3 => 1,
                4 => 5,
                5 => 2,
                6 => 6,
                7 => 3,
                _ => panic!("Woo"),
            };
        steps = new_location + password.len() - location;
    }
    
    rotate(password, Direction::Right, steps, false);
}

fn reverse(password: &mut Vec<char>, start: usize, end: usize) {
    let mut tmp: Vec<char> = Vec::with_capacity(end - start);
    for i in (start..end+1).rev() {
        tmp.push(password[i]);
    }
    for (index, value) in (start..end+1).enumerate() {
        password[value] = tmp[index];
    }
}

fn move_chr(password: &mut Vec<char>, from: usize, to: usize, rev: bool) {
    let f = if rev { to } else { from };
    let t = if rev { from } else { to };
    let chr = password.remove(f);
    password.insert(t, chr);
}

fn operate(op: &String, mut data: &mut Vec<char>, rev: bool) {
    //println!("{}", op);
    let cmd: Vec<&str> = op.split_whitespace().collect();
    match cmd[0] {
        "swap" => {
            match cmd[1] {
                "position" => swap_position(
                    &mut data,
                    cmd[2].parse().unwrap(),
                    cmd[5].parse().unwrap()),
                "letter" => swap_letter(
                    &mut data,
                    cmd[2].chars().next().unwrap(),
                    cmd[5].chars().next().unwrap()),
                _ => panic!("Bad command: {:?}", cmd),
            }
        },
        "rotate" => {
            match cmd[1] {
                "left" =>
                    rotate(
                        &mut data,
                        Direction::Left,
                        cmd[2].parse().unwrap(),
                        rev),
                "right" =>
                    rotate(
                        &mut data,
                        Direction::Right,
                        cmd[2].parse().unwrap(),
                        rev),
                "based" =>
                    rotate_based(
                        &mut data,
                        cmd[6].chars().next().unwrap(),
                        rev),
                _ => panic!("Bad command: {:?}", cmd),
            }
        },
        "reverse" => {
            reverse(
                &mut data,
                cmd[2].parse().unwrap(),
                cmd[4].parse().unwrap());
            
        },
        "move" => {
            move_chr(
                &mut data,
                cmd[2].parse().unwrap(),
                cmd[5].parse().unwrap(),
                rev);
        },
        _ => panic!("Bad command: {:?}", cmd),
    }
    //println!(" --> data: {}",
    //         String::from_iter(data.clone().into_iter()));
}

fn main() {
    println!("AOC 21");

    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "example".to_string()
    };
    let mut password: Vec<char> =  if args.len() > 2 {
        args[2].clone()
    } else {
        "abcde".to_string()
    }.chars().collect();
    let mut scramble: Vec<char> = password.clone();

    println!("Using file '{}' and input '{}'\n",
             file_name, String::from_iter(password.clone().into_iter()) );

    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let mut commands: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        commands.push(line.clone());
    }

    for command in &commands {
        operate(&command, &mut password, false);
    }
    println!("Scrambling {} produces: {}", args[2],
             String::from_iter(password.clone().into_iter()));

    for command in commands.iter().rev() {
        operate(&command, &mut scramble, true);
    }
    
    println!("Unscrambling {} produces: {}", args[2],
             String::from_iter(scramble.clone().into_iter()));

}
