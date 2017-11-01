use common;
use common::AocResultType;
use common::AocResult;

fn look_and_say(nums: &Vec<usize>) -> Vec<usize> {
    if nums.len() == 0 {
        return Vec::new();
    }
    let mut new_nums: Vec<usize> = Vec::new();
    let mut prev = nums[0];
    let mut nc = 1;
    for n in 1..nums.len() {
        let num = nums[n];
        if num == prev {
            nc += 1;
        } else {
            new_nums.push(nc);
            new_nums.push(prev);
            prev = num;
            nc = 1;
        }
    }
    new_nums.push(nc);
    new_nums.push(prev);

    new_nums
}

pub fn solve(input: Option<&str>) -> AocResult {

    let mut res: AocResult = AocResult {
        day: 10,
        phase_1: None,
        phase_2: None,
    };

    let input = match input {
        Some(s) => s.to_string(),
        None => common::get_input("Input", "3113322113"),
    };

    let mut numvec: Vec<usize> = Vec::new();
    match input.parse::<usize>() {
        Ok(_) => {
            for c in input.chars() {
                numvec.push(c.to_digit(10).unwrap() as usize);
            }
        }
        Err(_) => {
            println!("Not a number: {}", input);
            return res;
        }
    }

    println!("{:?}", numvec);
    for i in 1..51 {
        numvec = look_and_say(&numvec);
        println!("{}: {}", i, numvec.len());
        if i == 40 {
            res.phase_1 = Some(AocResultType::Usize(numvec.len()));
        }
        if i == 50 {
            res.phase_2 = Some(AocResultType::Usize(numvec.len()));
        }
    }

    res
}
