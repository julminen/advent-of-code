use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

fn is_tls(ip: &str) -> bool {
    // check for tls marker in string ("abba")
    
    let ip_len = ip.len();
    if ip_len < 4 {
        return false;
    }
    
    let mut str_vec: Vec<char> = Vec::with_capacity(ip_len);
    for c in ip.chars() {
        str_vec.push(c);
    }

    for i in 3..ip_len {
        if str_vec[i-3] == str_vec[i] && str_vec[i-2] == str_vec[i-1] && str_vec[i-3] != str_vec[i-2] {
            return true;
        }
    }
    
    return false;
}

fn tls_supported(parts: &Vec<(usize, &str)>, line: &str) -> bool {
    let mut previous_index = 0;
    let mut tls_found = false;
    let mut tls_supported = false;
    let strvec: Vec<char> = line.chars().collect();
    
    for i in parts {
        if i.1 == "]" {
            // hypernet section
            let mut hn = String::with_capacity(i.0 - previous_index);
            for i in previous_index..i.0 {
                hn.push(strvec[i]);
            }
            if is_tls(&hn) {
                tls_supported = false;
                break;
            }
        } else if !tls_found {
            // Check supernet, if tls support is not yet found
            let mut x = String::with_capacity(i.0 - previous_index);
            for i in previous_index..i.0 {
                x.push(strvec[i]);
            }
            if is_tls(&x) {
                tls_found = true;
                tls_supported = true;
            }
        }
        previous_index = i.0 + 1;
    }

    return tls_supported;
}

fn triplets(sn: &str) -> Vec<String> {
    let mut triplets: Vec<String> = Vec::new();
    let sn_len = sn.len();

    if sn_len < 3 {
        // println!("sn not > 3");
        return triplets;
    }

    let mut str_vec: Vec<char> = Vec::with_capacity(sn_len);
    for c in sn.chars() {
        str_vec.push(c);
    }

    for i in 1..(sn_len-1) {
        if str_vec[i-1] == str_vec[i+1] && str_vec[i-1] != str_vec[i] {
            let mut s = String::with_capacity(3);
            s.push(str_vec[i-1]);
            s.push(str_vec[i]);
            s.push(str_vec[i+1]);
            
            triplets.push(s);
        }
    }
    
    return triplets;
}

fn ssl_supported(parts: &Vec<(usize, &str)>, line: &str) -> bool {
    
    let strvec: Vec<char> = line.chars().collect();
    let mut previous_index = 0;
    
    let mut sn_triplets: Vec<String> = Vec::with_capacity(5);
    let mut hn_triplets: Vec<String> = Vec::with_capacity(5);
    
    for p in parts {
        let mut str = String::with_capacity(p.0 - previous_index);
        for c in previous_index..p.0 {
            str.push(strvec[c]);
        }
        
        let mut x = triplets(&str);
        if p.1 == "]" {
            // in hypernet section
            hn_triplets.append(&mut x);
        } else {
            // supernet section
            sn_triplets.append(&mut x);
        }
        
        previous_index = p.0+1;
    }
    // remove duplicates
    sn_triplets.sort();
    sn_triplets.dedup();
    hn_triplets.sort();
    hn_triplets.dedup();

    // modify hn triplets: aba -> bab
    for t in hn_triplets.iter_mut() {
        //println!("t: {}", t);
        let a = t.pop().unwrap();
        let b = t.pop().unwrap();
        let _ = t.pop().unwrap();
        t.push(b);
        t.push(a);
        t.push(b);
    }
    
    for sn in sn_triplets {
        if hn_triplets.contains(&sn) {
            return true;
        }
    }

    return false;
}


fn main() {
    
    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    println!("Reading {}", file_name);
    
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);
    
    let mut tls = 0;
    let mut ssl = 0;
    let mut total = 0;
    
    for line in reader.lines() {
        total += 1;
        let line = line.unwrap();
        // split by [ or ]
        let mut parts: Vec<(usize, &str)> = line.match_indices(|c| c == '[' || c == ']').collect();
        if line.len() != parts[parts.len()-1].0 {
            parts.push((line.len(), ""));
        }
        if tls_supported(&parts, &line) {
            tls += 1;
        }
        if ssl_supported(&parts, &line) {
            ssl += 1;
        }
    }
    println!("TLS support: {} / SSL support: {} / Total: {}", tls, ssl, total);
    
}
