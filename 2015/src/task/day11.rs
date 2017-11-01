use common;
use common::AocResultType;
use common::AocResult;
use std::fmt;

#[derive(Debug)]
struct PwdChar {
    chr: u8,
}

impl fmt::Display for PwdChar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char::from(self.chr))
    }
}

impl PwdChar {
    fn new(c: char) -> PwdChar {
        let x: u32 = u32::from(c);
        if x > u8::max_value() as u32 {
            panic!("Unsupported conversion from {} to u8", c);
        }
        PwdChar { chr: x as u8 }
    }
    fn next(&self, skip: &Vec<u8>) -> (PwdChar, bool) {
        let mut rolled = false;
        let mut next = self.chr;
        loop {
            next = if next + 1 > u32::from('z') as u8 {
                rolled = true;
                u32::from('a') as u8
            } else {
                next + 1
            };
            if !skip.contains(&next) {
                break;
            }
        }
        (PwdChar { chr: next }, rolled)
    }
}

#[derive(Debug)]
struct Password {
    letters: Vec<PwdChar>,
    bad_letters: Vec<u8>,
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::with_capacity(self.letters.len());
        for l in &self.letters {
            str.push(char::from(l.chr));
        }
        write!(f, "{}", str)
    }
}

impl Password {
    fn to_string(&self) -> String {
        let mut str = String::with_capacity(self.letters.len());
        for l in &self.letters {
            str.push(char::from(l.chr));
        }
        str
    }
    fn new(word: &str) -> Password {
        use std::iter::FromIterator;
        let chars: Vec<char> = Vec::from_iter(word.chars());
        if chars.len() != 8 {
            panic!("Bad length: {}", chars.len());
        }
        let mut letters = Vec::with_capacity(8);
        for i in 0..chars.len() {
            letters.push(PwdChar::new(chars[i]));
        }
        Password {
            letters: letters,
            bad_letters: vec![u32::from('i') as u8,
                              u32::from('o') as u8,
                              u32::from('l') as u8],
        }
    }
    fn advance(&mut self) {
        'main: for i in (0..self.letters.len()).rev() {
            let (next, rolled) = self.letters[i].next(&self.bad_letters);
            self.letters[i] = next;
            if !rolled {
                break 'main;
            }
        }
    }
    fn has_inc_triplet(&self) -> bool {
        for i in 2..self.letters.len() {
            let a = self.letters[i - 2].chr;
            let b = self.letters[i - 1].chr;
            let c = self.letters[i - 0].chr;
            if a + 1 == b && b + 1 == c {
                return true;
            }
        }
        false
    }
    fn has_good_letters(&self) -> bool {
        for i in &self.letters {
            if self.bad_letters.contains(&i.chr) {
                return false;
            }
        }
        true
    }
    fn has_two_different_pairs(&self) -> bool {
        for i in 1..self.letters.len() - 2 {
            if self.letters[i - 1].chr == self.letters[i].chr {
                for j in (i + 2)..self.letters.len() {
                    if self.letters[j - 1].chr == self.letters[j].chr &&
                       self.letters[j].chr != self.letters[i].chr {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn is_valid(&self) -> bool {
        self.has_inc_triplet() && self.has_good_letters() && self.has_two_different_pairs()
    }
}


pub fn solve(input: Option<&str>) -> AocResult {

    let mut res: AocResult = AocResult {
        day: 11,
        phase_1: None,
        phase_2: None,
    };

    let input = match input {
        Some(s) => s.to_string(),
        None => common::get_input("Input", "cqjxjnds"),
    };

    let mut pwd = Password::new(&input);

    // Phase 1
    for _ in 0..usize::max_value() {
        pwd.advance();
        if pwd.is_valid() {
            res.phase_1 = Some(AocResultType::String(pwd.to_string()));
            break;
        }
    }

    // Phase 2 is the next pwd
    for _ in 0..usize::max_value() {
        pwd.advance();
        if pwd.is_valid() {
            res.phase_2 = Some(AocResultType::String(pwd.to_string()));
            break;
        }
    }

    res
}
