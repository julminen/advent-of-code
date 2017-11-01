use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::collections::BTreeMap;


#[derive(Debug, Copy, Clone)]
enum OutputType {
    Bot,
    Output,
}

struct ValueSet {
    bot: usize,
    value: i32,
}

#[derive(Debug, Copy, Clone)]
struct BotInstruction {
    output_type: OutputType,
    output_id: usize,
}

impl BotInstruction {
    pub fn new(output_type: OutputType, output_id: usize) -> BotInstruction {
        BotInstruction {
            output_type: output_type,
            output_id: output_id,
        }
    }
    pub fn to_string(&self) -> String {
        match self.output_type {
            OutputType::Bot => {
                let mut s = String::from("bot ");
                s.push_str(&self.output_id.to_string());
                s
            },
            OutputType::Output => {
                let mut s = String::from("output ");
                s.push_str(&self.output_id.to_string());
                s
            },
        }
    }
}

struct Bot {
    items: Vec<i32>,
    hi_instruction: BotInstruction,
    low_instruction: BotInstruction,
}

impl Bot {
    pub fn new(low_type: OutputType, low_id: usize,
               high_type: OutputType, high_id: usize) -> Bot {
        Bot {
            items: Vec::with_capacity(2),
            hi_instruction: BotInstruction::new(high_type, high_id),
            low_instruction: BotInstruction::new(low_type, low_id),
        }
    }

    pub fn add_item(&mut self, item: i32) {
        if self.items.len() > 1 {
            panic!("Too many items");
        }
        if self.items.contains(&item) {
            panic!("Same item added twice");
        }
        self.items.push(item);
        self.items.sort();
    }

    pub fn can_act(&self) -> bool {
        return self.items.len() == 2;
    }
    
    pub fn to_string(&self) -> String {
        let mut s = String::from("high to ");
        s.push_str(&self.hi_instruction.to_string());
        s.push_str(", low to ");
        s.push_str(&self.low_instruction.to_string());
        s.push_str(", items: [");
        for i in self.items.iter() {
            s.push_str(&i.to_string());
            s.push_str(", ");
        }
        s.push_str("]");
        
        s
    }

}

fn main() {
    println!("AOC 10");

    let file_name = if env::args().len() > 1 {
        env::args().last().unwrap()
    } else {
        "input".to_string()
    };

    println!("Reading {}", file_name);
    let f = File::open(file_name).unwrap();
    let reader = BufReader::new(f);

    let mut bots: BTreeMap<usize, Bot> = BTreeMap::new();
    let mut outputs: BTreeMap<usize, i32> = BTreeMap::new();
    let mut values: Vec<ValueSet> = Vec::new();

    // Load instructions
    for line in reader.lines() {
        let line = line.unwrap();
        //println!("{}", line);
        let cmd: Vec<&str> = line.split(' ').collect();
        match cmd[0] {
            "bot" => {
                let bot_id: usize = cmd[1].parse().unwrap();
                let low_type = match cmd[5] {
                    "bot" => OutputType::Bot,
                    "output" => OutputType::Output,
                    _ => {
                        panic!("Bad output type! ");
                    }
                };
                let low_to: usize = cmd[6].parse().unwrap();
                let high_type = match cmd[10] {
                    "bot" => OutputType::Bot,
                    "output" => OutputType::Output,
                    _ => {
                        panic!("Bad output type! ");
                    }
                };
                let high_to: usize = cmd[11].parse().unwrap();

                let bot = Bot::new(low_type, low_to, high_type, high_to);

                bots.insert(bot_id, bot);
            },
            "value" => {
                let bot_id: usize = cmd[5].parse().unwrap();
                let value: i32 = cmd[1].parse().unwrap();
                values.push(ValueSet { bot: bot_id, value: value });
            },
            _ => {println!("Unknown command: {}", cmd[0])},
        }
    }
    // Load values to bots
    for vs in values {
        let mut bot: &mut Bot = bots.get_mut(&vs.bot).unwrap();
        bot.add_item(vs.value);
    }

    // act
    loop {
        let mut acted = false;
        let mut ab: Vec<usize> = Vec::new();
        for (id, bot) in bots.iter() {
            if bot.can_act() {
                ab.push(*id);
            }
        }
        for id in ab {
            let hi_key: usize;
            let lo_key: usize;
            let hi_value: i32;
            let lo_value: i32;
            let hi_ins: BotInstruction;
            let lo_ins: BotInstruction;
            {
                let mut bot = bots.get_mut(&id).unwrap();
                //println!("Bot {}: {}", id, bot.to_string());

                hi_key = bot.hi_instruction.output_id;
                lo_key = bot.low_instruction.output_id;
                hi_value = bot.items[1];
                lo_value = bot.items[0];
                hi_ins = bot.hi_instruction;
                lo_ins = bot.low_instruction;

                if hi_value == 61 && lo_value == 17 {
                    println!("!!!!! Bot {} compares {} and {} !!!!!", id, 61, 17);
                }
                
                bot.items.clear();
            }
            match hi_ins.output_type {
                OutputType::Bot => {
                    let mut bot = bots.get_mut(&hi_key).unwrap();
                    bot.add_item(hi_value);
                },
                OutputType::Output => {
                    outputs.insert(hi_key, hi_value);
                    println!("Bot {} outputs high {} to {}", id, hi_value, hi_key);
                },
            }
            match lo_ins.output_type {
                OutputType::Bot => {
                    let mut bot = bots.get_mut(&lo_key).unwrap();
                    bot.add_item(lo_value);
                },
                OutputType::Output => {
                    outputs.insert(lo_key, lo_value);
                    println!("Bot {} outputs low: {} to {}", id, lo_value, lo_key);
                },
            }
            acted = true;
        }
        if !acted {
            break;
        }
    };
    let mut acc = 1;
    for (k, v) in outputs.iter() {
        if *k < 3 as usize {
            acc = acc * v;
        }
        println!("{} -> {}", k, v);
    }
    println!("acc {}", acc);
    
}
