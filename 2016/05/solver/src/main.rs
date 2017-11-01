extern crate md5;

use std::env;

fn main() {

    let mut door_id: Vec<u8> = Vec::new();
    if env::args().len() > 1 {
        for argument in env::args() {
            door_id = argument.into_bytes();
        }
    } else {

        println!("No key given!");
        return;
    }
    
    //let door_id = b"abc";
    //let door_id = b"ffykfhsq";

    let mut key: Vec<u8> = Vec::with_capacity(50);
    for b in door_id.iter() {
        key.push(*b);
    }

    let num_tbl = b"0123456789";
    let mut keys_found = 0;
    let mut answer = [0; 8];
    let mut answer_found = [false; 8];
    let mut idx_vec: Vec<u8> = Vec::with_capacity(12);
    idx_vec.push(0);

    let mut carry: bool = false;
    loop {
        
        for c in idx_vec.iter() {
            key.push(num_tbl[*c as usize]);
        }
        let il = idx_vec.len();
        idx_vec[il-1] += 1;
        for c in idx_vec.iter_mut().rev() {
            if carry {
                *c = *c + 1;
                carry = false;
            }
            if *c > 9 {
                *c = 0;
                carry = true;
            }
        }
        if carry {
            idx_vec.insert(0, 1);
            carry = false;
        }
        
        //println!("{:?}", key);
        let hash = md5::compute(key.as_slice());
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] & 0xF8 == 0x00 {
            let a_idx = hash[2] & 0x07;
            if !answer_found[a_idx as usize] {
                let a_val = hash[3]>>4;
                answer_found[a_idx as usize] = true;
                println!("{} = {:x}", a_idx, a_val);
                answer[a_idx as usize] = a_val;
                keys_found += 1;
                if keys_found == answer.len() {
                    break;
                }
            }
        }
        key.truncate(door_id.len());
    }

    for b in answer.iter() {
        print!("{:x}", b);
    }
    println!("");
}
