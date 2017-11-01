extern crate md5;
extern crate scoped_threadpool;
extern crate num_cpus;
use scoped_threadpool::Pool;

use std::io;
use std::io::prelude::*;
use std::time;
use std::env;


//
fn get_input(prompt: &str, default: &str) -> String {
    print!("{} [{}]: ", prompt, default);
    match io::stdout().flush() {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    let mut buffer = String::new();

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer = buffer.trim().to_string();
            if buffer == "" {
                buffer = default.to_string();
            }
        }
        Err(error) => println!("error: {}", error),
    }

    buffer
}

fn get_input_usize(prompt: &str, default: usize) -> usize {
    print!("{} [{}]: ", prompt, default);
    match io::stdout().flush() {
        Ok(_) => {}
        Err(error) => println!("error: {}", error),
    }
    let mut buffer = String::new();
    let res;

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                buffer = buffer.trim().to_string();
                if buffer == "" {
                    res = default;
                    break;
                } else {
                    match buffer.parse::<usize>() {
                        Ok(i) => {
                            res = i;
                            break;
                        }
                        Err(_) => println!("Cannot parse {} as usize", buffer),
                    };
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }

    res
}


fn format_hash(hash: &[u8; 16]) -> [u8; 32] {
    let mut w = [0; 32];
    for i in 0..16 {
        w[i * 2] = match hash[i] >> 4 {
            0x00 => 48,
            0x01 => 49,
            0x02 => 50,
            0x03 => 51,
            0x04 => 52,
            0x05 => 53,
            0x06 => 54,
            0x07 => 55,
            0x08 => 56,
            0x09 => 57,
            0x0a => 97,
            0x0b => 98,
            0x0c => 99,
            0x0d => 100,
            0x0e => 101,
            0x0f => 102,
            _ => panic!("wtf {}", hash[i] >> 4),
        };
        w[i * 2 + 1] = match hash[i] & 0x0f {
            0x00 => 48,
            0x01 => 49,
            0x02 => 50,
            0x03 => 51,
            0x04 => 52,
            0x05 => 53,
            0x06 => 54,
            0x07 => 55,
            0x08 => 56,
            0x09 => 57,
            0x0a => 97,
            0x0b => 98,
            0x0c => 99,
            0x0d => 100,
            0x0e => 101,
            0x0f => 102,
            _ => panic!("wtf {}", hash[i] & 0x0f),
        };
    }
    w
}

fn compute_hash(input: &Vec<u8>, stretch: usize) -> [u8; 16] {
    let mut hash = md5::compute(input.as_slice());
    for _ in 0..stretch {
        hash = md5::compute(&format_hash(&hash));
    }
    hash
}


fn find_triplet(hash: &[u8; 16]) -> Option<u8> {
    // by byte halves
    // 12 34
    // 2==3 and 2==1 or 2==4
    for i in 1..hash.len() {
        if hash[i-1] & 0x0f == hash[i] >> 4 &&     // 2 == 3 and
            (hash[i-1] & 0x0f == hash[i-1] >> 4 || // (2 == 1 or
             hash[i-1] & 0x0f == hash[i] & 0x0f)
        //  2 == 4)
        {
            return Some(hash[i] >> 4);
        }
    }
    None
}

fn has_quintet(hash: &[u8; 16], needle: u8) -> bool {
    // 12 34 56
    // 2=needle and 2=3=4=5 and 1=2 or 6=2
    for i in 2..hash.len() {
        if hash[i - 2] & 0x0f == needle {
            if hash[i-2] & 0x0f == hash[i-1] >> 4 &&    // 2==3 and
                hash[i-2] & 0x0f == hash[i-1] & 0x0f && // 2==4 and
                hash[i-2] & 0x0f == hash[i] >> 4 &&     // 2==5 and
                (hash[i-2] & 0x0f == hash[i-2] >> 4 ||  // (2==1 or
                 hash[i-2] &0x0f == hash[i] & 0x0f)
            //  2==6)
            {
                return true;
            }
        }
    }
    false
}

fn get_next_triplet_idx(salt: &str,
                        salt_idx: usize,
                        stretch: usize,
                        cache: &mut Vec<(usize, u8, [u8; 16])>,
                        cache_idx: usize,
                        pool: &mut Pool)
                        -> usize {
    // search cache
    for ci in cache_idx..cache.len() {
        if cache[ci].0 >= salt_idx {
            return ci;
        }
    }
    let mut ci = 0;
    let mut found = false;
    // not found in cache, so compute
    let jobs = 256;
    let mut ix = salt_idx;
    let mut hashes: Vec<[u8; 16]> = vec![[0; 16]; jobs];
    let mut inputs: Vec<Vec<u8>> = Vec::with_capacity(4);
    for _ in salt_idx..usize::max_value() {
        inputs.truncate(0);
        for j in 0..jobs {
            inputs.push(salt.to_string().into_bytes());
            inputs[j].append(&mut (ix+j).to_string().into_bytes().clone());
        }
        
        pool.scoped(|scope| {
            let mut x = 0;
            for h in &mut hashes {
            let inp = &inputs[x];
            scope.execute(move || {
            *h = compute_hash(inp, stretch);
            });
            x += 1;
            }
        });

        for (hi, h) in hashes.iter().enumerate() {
        match find_triplet(&h) {
            Some(t) => {
                cache.push((ix+hi, t, *h));
                if ! found {
                ci = cache.len() - 1;
                found = true;
                }
            }
            None => {}
        }
        }
        if found {
            break;
        }
        ix += jobs;
    }
    ci
}

fn find_keys(salt: &str, stretch: usize) -> Vec<(usize, usize, [u8; 16])> {

    let mut keys: Vec<(usize, usize, [u8; 16])> = Vec::with_capacity(64);
    let mut valid_keys = 0;
    let mut triplet_cache: Vec<(usize, u8, [u8; 16])> = Vec::with_capacity(5000);

    let cpus = num_cpus::get();
    println!("{} CPUs detected", cpus);
    let mut pool = Pool::new(cpus as u32);

    let mut salt_idx = 0;
    let mut triplet_idx = 0;
    'triplet: loop {
        triplet_idx = get_next_triplet_idx(&salt,
                                           salt_idx,
                                           stretch,
                                           &mut triplet_cache,
                                           triplet_idx,
                                           &mut pool);
        salt_idx = triplet_cache[triplet_idx].0;
        let search_byte = triplet_cache[triplet_idx].1;

        let mut quintet_salt_idx = salt_idx;
        let mut quintet_idx = triplet_idx;
        'quintet: loop {
            quintet_salt_idx += 1;
            if quintet_salt_idx > salt_idx + 1000 {
                break 'quintet;
            }
            quintet_idx = get_next_triplet_idx(&salt,
                                               quintet_salt_idx,
                                               stretch,
                                               &mut triplet_cache,
                                               quintet_idx,
                                               &mut pool);
            quintet_salt_idx = triplet_cache[quintet_idx].0;

            if has_quintet(&triplet_cache[quintet_idx].2, search_byte) {
                valid_keys += 1;
                keys.push((valid_keys, salt_idx, triplet_cache[triplet_idx].2));
                if valid_keys >= 64 {
                    break 'triplet;
                }
                break 'quintet;
            }
        }
        salt_idx += 1;
    }

    keys
}

fn get_parameter(args: &Vec<String>, flag: &str) -> Option<String> {
    for i in 0..args.len() - 1 {
        if flag == args[i] {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn main() {

    // input zpqevtbw : 16106 / 22423

    let default_key = "zpqevtbw";

    println!("Advent of Code 2016 / 14");

    let args: Vec<String> = env::args().collect();

    let salt = match get_parameter(&args, "-i") {
        Some(s) => s,
        None => get_input("Salt", default_key),
    };
    let stretch = match get_parameter(&args, "-s") {
        Some(s) => {
            match s.parse::<usize>() {
                Ok(u) => u,
                Err(_) => get_input_usize("Bad stretch as option, retry", 2016),
            }
        }
        None => get_input_usize("Stretch", 2016),
    };

    println!("Salt: {}, stretch: {}", salt, stretch);

    let start = time::SystemTime::now();
    let keys = find_keys(&salt, stretch);
    let last_key = keys.last().unwrap();

    print!("Key {} was found at index {} and is ",
           last_key.0,
           last_key.1);
    for b in last_key.2.iter() {
        print!("{:02x}", b);
    }
    println!("");

    match start.elapsed() {
        Ok(elapsed) => {
            println!("Elapsed time: {}.{:09}",
                     elapsed.as_secs(),
                     elapsed.subsec_nanos())
        }
        Err(e) => println!("Cannot measure time: {}", e),
    }
}
