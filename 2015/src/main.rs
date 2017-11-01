mod common;
mod task;

fn main() {
    println!("Advent of Code 2015");

    let mut last_day = 0;
    loop {
        last_day += 1;
        let option = common::get_input("Day or 'q'", &last_day.to_string());
        if option == "q" {
            println!("Bye");
            break;
        }
        match option.parse() {
            Ok(day) => {
                let res: Option<common::AocResult> = match day {
                    1 => Some(task::day01::solve(None)),
                    2 => Some(task::day02::solve(None)),
                    3 => Some(task::day03::solve(None)),
                    4 => Some(task::day04::solve(None)),
                    5 => Some(task::day05::solve(None)),
                    6 => Some(task::day06::solve(None)),
                    7 => Some(task::day07::solve(None)),
                    8 => Some(task::day08::solve(None)),
                    9 => Some(task::day09::solve(None)),
                    10 => Some(task::day10::solve(None)),
                    11 => Some(task::day11::solve(None)),
                    12 => Some(task::day12::solve(None)),
                    13 => Some(task::day13::solve(None)),
                    14 => Some(task::day14::solve(None, None)),
                    15 => Some(task::day15::solve(None)),
                    16 => Some(task::day16::solve(None)),
                    17 => Some(task::day17::solve(None)),
                    18 => Some(task::day18::solve(None)),
                    19 => Some(task::day19::solve(None)),
                    20 => Some(task::day20::solve(None)),
                    21 => Some(task::day21::solve(None, None)),
                    22 => Some(task::day22::solve(None, None)),
                    23 => Some(task::day23::solve(None)),
                    24 => Some(task::day24::solve(None)),
                    25 => Some(task::day25::solve(None)),
                    _ => None,
                };
                match res {
                    Some(r) => println!("{}", r),
                    None => println!("{}: Unimplemented", day),
                };
                last_day = day;
            }
            Err(_) => println!("Bad input, try again"),
        };
    }
}

#[test]
fn day_01() {
    let res = task::day01::solve(Some("input"));
    assert_eq!(res.day, 1);
    assert_eq!(res.phase_1.unwrap(), common::AocResultType::Isize(280));
    assert_eq!(res.phase_2.unwrap(), common::AocResultType::Usize(1797));
}
#[test]
fn day_02() {
    let res = task::day02::solve(Some("input"));
    assert_eq!(res.day, 2);
    assert_eq!(res.phase_1.unwrap(), common::AocResultType::Usize(1588178));
    assert_eq!(res.phase_2.unwrap(), common::AocResultType::Usize(3783758));
}
#[test]
fn day_03() {
    let res = task::day03::solve(Some("input"));
    assert_eq!(res.day, 3);
    assert_eq!(res.phase_1.unwrap(), common::AocResultType::Usize(2565));
    assert_eq!(res.phase_2.unwrap(), common::AocResultType::Usize(2639));
}
#[test]
fn day_04() {
    let res = task::day04::solve(Some("yzbqklnj"));
    assert_eq!(res.day, 4);
    assert_eq!(res.phase_1.unwrap(), common::AocResultType::Usize(282749));
    assert_eq!(res.phase_2.unwrap(), common::AocResultType::Usize(9962624));
}
#[test]
fn day_05() {
    let res = task::day05::solve(Some("input"));
    assert_eq!(res.day, 5);
    assert_eq!(res.phase_1.unwrap(), common::AocResultType::Usize(236));
    assert_eq!(res.phase_2.unwrap(), common::AocResultType::Usize(51));
}
#[test]
fn day_06() {
    let res = task::day06::solve(Some("input"));
    assert_eq!(res.day, 6);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(400410)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(15343601)));
}
#[test]
fn day_07() {
    let res = task::day07::solve(Some("input"));
    assert_eq!(res.day, 7);
    assert_eq!(res.phase_1, Some(common::AocResultType::U16(16076)));
    assert_eq!(res.phase_2, Some(common::AocResultType::U16(2797)));
}
#[test]
fn day_08() {
    let res = task::day08::solve(Some("input"));
    assert_eq!(res.day, 8);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(1333)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(2046)));
}
#[test]
fn day_09() {
    let res = task::day09::solve(Some("input"));
    assert_eq!(res.day, 9);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(251)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(898)));
}
#[test]
fn day_10() {
    let res = task::day10::solve(Some("3113322113"));
    assert_eq!(res.day, 10);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(329356)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(4666278)));
}
#[test]
fn day_11() {
    use common::AocResultType;
    let res = task::day11::solve(Some("cqjxjnds"));
    assert_eq!(res.day, 11);
    assert_eq!(res.phase_1,
               Some(AocResultType::String(String::from("cqjxxyzz"))));
    assert_eq!(res.phase_2,
               Some(AocResultType::String(String::from("cqkaabcc"))));
}
#[test]
fn day_12() {
    let res = task::day12::solve(Some("input"));
    assert_eq!(res.day, 12);
    assert_eq!(res.phase_1, Some(common::AocResultType::Isize(156366)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Isize(96852)));
}
#[test]
fn day_13() {
    let res = task::day13::solve(Some("input"));
    assert_eq!(res.day, 13);
    assert_eq!(res.phase_1, Some(common::AocResultType::Isize(733)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Isize(725)));
}
#[test]
fn day_14() {
    let res = task::day14::solve(Some("input"), Some(2503));
    assert_eq!(res.day, 14);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(2655)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(1059)));
}
#[test]
fn day_15() {
    let res = task::day15::solve(Some("input"));
    assert_eq!(res.day, 15);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(18965440)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(15862900)));
}
#[test]
fn day_16() {
    let res = task::day16::solve(Some("input"));
    assert_eq!(res.day, 16);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(40)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(241)));
}
#[test]
fn day_17() {
    let res = task::day17::solve(Some("input"));
    assert_eq!(res.day, 17);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(1304)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(18)));
}
#[test]
fn day_18() {
    let res = task::day18::solve(Some("input"));
    assert_eq!(res.day, 18);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(821)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(886)));
}
#[test]
fn day_19() {
    let res = task::day19::solve(Some("input"));
    assert_eq!(res.day, 19);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(535)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(212)));
}
#[test]
fn day_20() {
    let res = task::day20::solve(Some("33100000"));
    assert_eq!(res.day, 20);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(776160)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(786240)));
}
#[test]
fn day_21() {
    let res = task::day21::solve(Some("boss"), Some("player"));
    assert_eq!(res.day, 21);
    assert_eq!(res.phase_1, Some(common::AocResultType::Usize(121)));
    assert_eq!(res.phase_2, Some(common::AocResultType::Usize(201)));
}
