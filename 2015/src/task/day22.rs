use common;
// use common::AocResultType;
use common::AocResult;

use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
//use std::cmp::Ordering;
//use std::cmp;
//use std::fmt;

use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Character {
    hit_points: isize,
    damage: isize,
    mana: isize,
    armor: isize,
    spent_mana: usize,
}
impl Character {
    fn new(map: &HashMap<String, isize>) -> Character {
        let hit_points = match map.get("Hit Points") {
            Some(hp) => *hp,
            None => 0
        };
        let damage = match map.get("Damage") {
            Some(d) => *d,
            None => 0
        };
        let mana = match map.get("Mana") {
            Some(m) => *m,
            None => 0
        };
        Character {
            hit_points,
            damage,
            mana,
            armor: 0,
            spent_mana: 0
        }
    }
}

struct GameContext {
    round: usize,
    player: Character,
    boss: Character,
    active_effects: Vec<Spell>,
    
}

/*
  • Magic Missile costs 53 mana. It instantly does 4 damage.
  • Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit
    points.
  • Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is
    active, your armor is increased by 7.
  • Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the
    start of each turn while it is active, it deals the boss 3 damage.
  • Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the
    start of each turn while it is active, it gives you 101 new mana.
*/

#[derive(Debug)]
enum SpellType {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Debug)]
struct Spell {
    spell_type: SpellType,
    effect_time: usize,
    cost: usize,
}

fn cast_spell(spell: &mut Spell, player: &mut Character, boss: &mut Character) -> bool {
    match spell.spell_type {
        SpellType::MagicMissile => {
            boss.hit_points -= 4;
        },
        SpellType::Drain => {
            player.hit_points += 2;
            boss.hit_points -= 2;
        },
        SpellType::Shield => { // effect time 6
            if spell.effect_time == 6 {
                player.armor += 7;
            } else if spell.effect_time == 0 {
                player.armor -= 7;
            }
        },
        SpellType::Poison => { // effect time 6
            if spell.effect_time <= 5 {
                boss.hit_points -= 3;
            }
        },
        SpellType::Recharge => { // effect time 5
            if spell.effect_time <= 4 {
                player.mana += 101;
            }
        },
    }
    if spell.effect_time >= 1 {
        spell.effect_time -= 1;
        true
    } else {
        false
    }
}

impl Spell {
    fn new(spell_type: SpellType) -> Spell {
        match spell_type {
            SpellType::MagicMissile => {
                Spell {
                    spell_type: SpellType::MagicMissile,
                    effect_time: 0,
                    cost: 53
                }
            },
            SpellType::Drain => {
                Spell {
                    spell_type: SpellType::Drain,
                    effect_time: 0,
                    cost: 73
                }
            },
            SpellType::Shield => {
                Spell {
                    spell_type: SpellType::Shield,
                    effect_time: 6,
                    cost: 113
                }
            },
            SpellType::Poison => {
                Spell {
                    spell_type: SpellType::Poison,
                    effect_time: 6,
                    cost: 173
                }
            },
            SpellType::Recharge => {
                Spell {
                    spell_type: SpellType::Recharge,
                    effect_time: 5,
                    cost: 229
                }
            },
        }
    }
    fn reset(&mut self) {
        self.effect_time = 
        match self.spell_type {
            SpellType::Shield | SpellType::Poison => {
                6
            },
            SpellType::Recharge => {
                5
            },
            _ => 0
        }
    }
}


fn load_character_from_file(file_name: &str) -> Character {
    let reader = BufReader::new(File::open(file_name).unwrap());
    let mut attrs: HashMap<String, isize> = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let tokens: Vec<&str> = line.split(": ").collect();
        if tokens.len() >= 2 {
            match isize::from_str(tokens[1]) {
                Ok(ok) => {
                    attrs.insert(tokens[0].to_string(), ok);
                } 
                Err(_) => {}
            }
        }
    }
    Character::new(&attrs)
}

fn game_loop(player: &mut Character, boss: &mut Character) {
    // Create spells
    let mut missile = Spell::new(SpellType::MagicMissile);
    let mut drain = Spell::new(SpellType::Drain);
    let mut shield = Spell::new(SpellType::Shield);
    let mut poison = Spell::new(SpellType::Poison);
    let mut recharge = Spell::new(SpellType::Recharge);

    let mut effects: Vec<Spell> = Vec::with_capacity(5);
    
    // handle effect spells
    // check if player or boss has hit_points <= 0
    // handle player / boss actions
}


pub fn solve(boss_file: Option<&str>, player_file: Option<&str>) -> AocResult {

    let path = "input/22/";
    let mut boss_file_name = String::from(path);
    match boss_file {
        Some(name) => boss_file_name.push_str(name),
        None => {
            let name = common::get_input("Boss file name", "boss");
            boss_file_name.push_str(&name);
        }
    }
    let mut boss = load_character_from_file(&boss_file_name);

    let mut player_file_name = String::from(path);
    match player_file {
        Some(name) => player_file_name.push_str(name),
        None => {
            let name = common::get_input("Player file name", "player");
            player_file_name.push_str(&name);
        }
    }
    let mut player = load_character_from_file(&player_file_name);

    println!("Player: {:?}", player); 
    println!("Boss: {:?}", boss);
    
    let mut missile = Spell::new(SpellType::MagicMissile);
    let mut drain = Spell::new(SpellType::Drain);
    let mut shield = Spell::new(SpellType::Shield);
    let mut poison = Spell::new(SpellType::Poison);
    let mut recharge = Spell::new(SpellType::Recharge);

    println!("{:?}", missile);
    println!("{:?}", drain);
    println!("{:?}", shield);
    println!("{:?}", poison);
    println!("{:?}", recharge);
    println!("missile: {}", cast_spell(&mut missile, &mut player, &mut boss));
    println!("drain: {}", cast_spell(&mut drain, &mut player, &mut boss));
    println!("shield: {}", cast_spell(&mut shield, &mut player, &mut boss));
    println!("poison: {}", cast_spell(&mut poison, &mut player, &mut boss));
    println!("recharge: {}", cast_spell(&mut recharge, &mut player, &mut boss));
    println!("{:?}", missile);
    println!("{:?}", drain);
    println!("{:?}", shield);
    println!("{:?}", poison);
    println!("{:?}", recharge);

    println!("Player: {:?}", player); 
    println!("Boss: {:?}", boss);

	game_loop(&mut player, &mut boss);
    
    AocResult {
        day: 22,
        phase_1: None,
        phase_2: None,
    }
}
