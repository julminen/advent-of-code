extern crate md5;
use common;
use common::AocResultType;
use common::AocResult;

pub fn solve(input: Option<&str>) -> AocResult {

    let mut res: AocResult = AocResult {
        day: 4,
        phase_1: None,
        phase_2: None,
    };

    let mut key = Vec::from(match input {
                                    Some(s) => s.to_string(),
                                    None => common::get_input("Input", "yzbqklnj"),
                                }
                                .as_bytes());
    let key_len = key.len();


    for i in 0..usize::max_value() {
        key.truncate(key_len);
        for c in i.to_string().chars() {
            key.push(c as u8);
        }
        let h = md5::compute(key.as_slice());
        if h[0] == 0 && h[1] == 0 && h[2] & 0xf0 == 0 && res.phase_1 == None {
            res.phase_1 = Some(AocResultType::Usize(i));
        }
        if h[0] == 0 && h[1] == 0 && h[2] == 0 && res.phase_2 == None {
            res.phase_2 = Some(AocResultType::Usize(i));
            break;
        }
    }

    res
}
