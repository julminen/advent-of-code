use common;
// use common::AocResultType;
use common::AocResult;

pub fn solve(input: Option<&str>) -> AocResult {

    let input = match input {
        Some(s) => s.to_string(),
        None => common::get_input("Input", "xxxx"),
    };
    println!("Input {}", input);

    AocResult {
        day: 23,
        phase_1: None,
        phase_2: None,
    }
}
