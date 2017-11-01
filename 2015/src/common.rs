use std::fmt;
use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
pub enum AocResultType {
    Isize(isize),
    Usize(usize),
    U16(u16),
    String(String),
}

impl fmt::Display for AocResultType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match self {
                   &AocResultType::Isize(ref a) => a.to_string(),
                   &AocResultType::Usize(ref a) => a.to_string(),
                   &AocResultType::U16(ref a) => a.to_string(),
                   &AocResultType::String(ref a) => a.clone(),
               })
    }
}

#[derive(Debug)]
pub struct AocResult {
    pub day: usize,
    pub phase_1: Option<AocResultType>,
    pub phase_2: Option<AocResultType>,
}

impl fmt::Display for AocResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Day {}:\n\tPhase 1: {}\n\tPhase 2: {}",
               self.day,
               match self.phase_1 {
                   Some(ref x) => x.to_string(),
                   None => "<no answer>".to_string(),
               },
               match self.phase_2 {
                   Some(ref x) => x.to_string(),
                   None => "<no answer>".to_string(),
               },
        )
    }
}



pub fn get_input(prompt: &str, default: &str) -> String {
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
