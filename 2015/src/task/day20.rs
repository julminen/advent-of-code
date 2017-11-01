use common;
use common::AocResultType;
use common::AocResult;

use std::str::FromStr;

// Improvement: use cached primes instead
// This checks divisibility from 1 to sqrt(house).
// If house number is divisible, then add that number and house
// number divided by divisor (multiplied by present count) to total.
//
fn presents(house: usize, presents: usize, max_visits: Option<usize>) -> usize {
    let mut total = 0;
    let up = ((f64::from(house as u32)).sqrt() as usize) + 1;
    for i in 1..up {
        if house % i == 0 {
            match max_visits {
                None => {
                    total += i * presents;
                    if i != house / i {
                        total += (house / i) * presents;
                    }
                }
                Some(max) => {
                    // Max number of visits is defined, so filter out tired elves
                    if i * max >= house {
                        total += i * presents;
                    }
                    if i != house / i && (house / i) * max >= house {
                        total += (house / i) * presents;
                    }
                }
            }
        }
    }
    total
}

pub fn solve(input: Option<&str>) -> AocResult {

    let input = match input {
        Some(s) => s.to_string(),
        None => common::get_input("Input", "33100000"),
    };
    let limit: usize = usize::from_str(&input).unwrap();

    // phase 1
    let mut house_1 = 0;
    for house in 1..usize::max_value() {
        let presents = presents(house, 10, None);
        if presents >= limit {
            //println!("Answer 1: House {} got {} presents", house, presents);
            house_1 = house;
            break;
        }
    }

    // phase 2
    let mut house_2 = 0;
    for house in 1..usize::max_value() {
        let presents = presents(house, 11, Some(50));
        if presents >= limit {
            //println!("Answer 2: House {} got {} presents", house, presents);
            house_2 = house;
            break;
        }
    }

    AocResult {
        day: 20,
        phase_1: Some(AocResultType::Usize(house_1)),
        phase_2: Some(AocResultType::Usize(house_2)),
    }
}
