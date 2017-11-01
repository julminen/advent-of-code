use common;
use common::AocResultType;
use common::AocResult;

use std::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::Ordering;
use std::cmp;
use std::fmt;

use std::collections::HashMap;
use std::collections::BTreeSet;

type Store = HashMap<ItemType, BTreeSet<Item>>;

#[derive(Debug, Clone)]
struct Character {
    hit_points: isize,
    damage: usize,
    armor: usize,
    cost: usize,
    it_weapon: Option<Item>,
    it_armor: Option<Item>,
    it_rings: (Option<Item>, Option<Item>),
}

impl Character {
    fn new(map: &HashMap<String, isize>) -> Option<Character> {
        if map.contains_key("Hit Points") && map.contains_key("Damage") &&
           map.contains_key("Armor") {
            Some(Character {
                hit_points: *map.get("Hit Points").unwrap(),
                damage: *map.get("Damage").unwrap() as usize,
                armor: *map.get("Armor").unwrap() as usize,
                cost: 0,
                it_weapon: None,
                it_armor: None,
                it_rings: (None, None),
            })
        } else {
            None
        }
    }
    // Set weapon according to game rules
    fn set_weapon(&mut self, item: Option<Item>) {
        match self.it_weapon {
            Some(ref mut w) => {
                self.damage -= w.damage;
                self.armor -= w.armor;
                self.cost -= w.cost;
            }
            None => {}
        }
        match item {
            Some(ref w) => {
                if w.item_type != ItemType::Weapon {
                    panic!("Cannot use {:?} as weapon!", w.item_type);
                }
                self.damage += w.damage;
                self.armor += w.armor;
                self.cost += w.cost;
            }
            None => {}
        }
        self.it_weapon = item;
    }
    // Set armor according to game rules
    fn set_armor(&mut self, item: Option<Item>) {
        match self.it_armor {
            Some(ref mut a) => {
                self.damage -= a.damage;
                self.armor -= a.armor;
                self.cost -= a.cost;
            }
            None => {}
        }
        match item {
            Some(ref a) => {
                if a.item_type != ItemType::Armor {
                    panic!("Cannot use {:?} as armor!", a.item_type);
                }
                self.damage += a.damage;
                self.armor += a.armor;
                self.cost += a.cost;
            }
            None => {}
        }
        self.it_armor = item;
    }
    // Set rings according to game rules
    fn set_rings(&mut self, rings: (Option<Item>, Option<Item>)) {
        match self.it_rings.0 {
            Some(ref mut r) => {
                self.damage -= r.damage;
                self.armor -= r.armor;
                self.cost -= r.cost;
            }
            None => {}
        }
        match self.it_rings.1 {
            Some(ref mut r) => {
                self.damage -= r.damage;
                self.armor -= r.armor;
                self.cost -= r.cost;
            }
            None => {}
        }
        match rings.0 {
            Some(ref r) => {
                if r.item_type != ItemType::Ring {
                    panic!("Cannot use {:?} as ring!", r.item_type);
                }
                self.damage += r.damage;
                self.armor += r.armor;
                self.cost += r.cost;
            }
            None => {}
        }
        match rings.1 {
            Some(ref r) => {
                if r.item_type != ItemType::Ring {
                    panic!("Cannot use {:?} as ring!", r.item_type);
                }
                self.damage += r.damage;
                self.armor += r.armor;
                self.cost += r.cost;
            }
            None => {}
        }
        self.it_rings = (rings.0, rings.1);
    }
}

#[derive(Clone,Debug,Hash,Eq,PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug,Eq,Clone)]
struct Item {
    item_type: ItemType,
    name: String,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    fn new(item_type: ItemType, name: &str, cost: usize, damage: usize, armor: usize) -> Item {
        Item {
            item_type: item_type,
            name: name.to_string(),
            cost: cost,
            damage: damage,
            armor: armor,
        }
    }
    fn new_weapon(name: &str, cost: usize, damage: usize, armor: usize) -> Item {
        Item::new(ItemType::Weapon, name, cost, damage, armor)
    }
    fn new_armor(name: &str, cost: usize, damage: usize, armor: usize) -> Item {
        Item::new(ItemType::Armor, name, cost, damage, armor)
    }
    fn new_ring(name: &str, cost: usize, damage: usize, armor: usize) -> Item {
        Item::new(ItemType::Ring, name, cost, damage, armor)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_store() -> Store {
    let store_contents = r"
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";
    let mut weapons: BTreeSet<Item> = BTreeSet::new();
    let mut armors: BTreeSet<Item> = BTreeSet::new();
    let mut rings: BTreeSet<Item> = BTreeSet::new();

    let mut ct = ItemType::Weapon;
    for line in store_contents.lines() {
        if line.len() == 0 {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 0 {
            println!("Bad line: {}, len: {}", line, line.len());
            continue;
        }
        use self::ItemType::*;
        match tokens[0] {
            "Weapons:" => ct = Weapon,
            "Armor:" => ct = Armor,
            "Rings:" => ct = Ring,
            _ => {
                match ct {
                    Weapon => {
                        weapons.insert(Item::new_weapon(tokens[0],
                                                        usize::from_str(tokens[1]).unwrap(),
                                                        usize::from_str(tokens[2]).unwrap(),
                                                        usize::from_str(tokens[3]).unwrap()));
                    }
                    Armor => {
                        armors.insert(Item::new_armor(tokens[0],
                                                      usize::from_str(tokens[1]).unwrap(),
                                                      usize::from_str(tokens[2]).unwrap(),
                                                      usize::from_str(tokens[3]).unwrap()));
                    }
                    Ring => {
                        rings.insert(Item::new_ring((tokens[0].to_owned() + " " + tokens[1])
                                                        .as_str(),
                                                    usize::from_str(tokens[2]).unwrap(),
                                                    usize::from_str(tokens[3]).unwrap(),
                                                    usize::from_str(tokens[4]).unwrap()));
                    }
                }
            }
        }
    }

    let mut contents: Store = HashMap::new();

    contents.insert(ItemType::Weapon, weapons);
    contents.insert(ItemType::Armor, armors);
    contents.insert(ItemType::Ring, rings);

    contents
}

fn load_file(file_name: &str) -> Option<Character> {
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

#[derive(Debug)]
enum Winner {
    Player,
    Boss,
}

fn fight(player: &Character, boss: &Character) -> Winner {
    let mut player_hp = player.hit_points;
    let mut boss_hp = boss.hit_points;
    let player_hit = cmp::max(1, player.damage as isize - boss.armor as isize);
    let boss_hit = cmp::max(1, boss.damage as isize - player.armor as isize);

    loop {
        // Player hits first
        boss_hp -= player_hit;
        //println!("The player deals {}-{} = {} damage; the boss goes down to {} hit points", player.damage, boss.armor, player_hit, boss_hp);
        if boss_hp <= 0 {
            return Winner::Player;
        }
        // Boss hits
        player_hp -= boss_hit;
        //println!("The boss deals {}-{} = {} damage; the player goes down to {} hit points", boss.damage, player.armor, boss_hit, player_hp);
        if player_hp <= 0 {
            return Winner::Boss;
        }
    }
}

fn play(store: &Store, player: &Character, boss: &Character) -> (usize, usize) {
    let mut min_gold = usize::max_value();
    let mut max_gold = usize::min_value();

    let mut player = player.clone();

    let weapons = store.get(&ItemType::Weapon).unwrap();
    let mut armors = store.get(&ItemType::Armor).unwrap().clone();
    armors.insert(Item::new_armor("No armor", 0, 0, 0));
    let armors = armors;

    let mut rings = store.get(&ItemType::Ring).unwrap().clone();
    rings.insert(Item::new_ring("No ring", 0, 0, 0));
    for w in weapons {
        for a in &armors {
            for lr in &rings {
                for rr in &rings {
                    if lr.name != "No ring" && lr == rr {
                        continue;
                    }
                    player.set_weapon(Some(w.clone()));
                    player.set_armor(Some(a.clone()));
                    player.set_rings((Some(lr.clone()), Some(rr.clone())));
                    let winner = fight(&player, &boss);
                    match winner {
                        Winner::Player => {
                            if player.cost < min_gold {
                                min_gold = player.cost;
                                //println!("New min: {} using weapon {}, armor {}, ring {} and ring {}", min_gold, w, a, lr, rr);
                            }
                        }
                        Winner::Boss => {
                            if player.cost > max_gold {
                                max_gold = player.cost;
                                //println!("New max: {} using weapon {}, armor {}, ring {} and ring {}", max_gold, w, a, lr, rr);
                            }
                        }
                    }
                }
            }
        }
    }
    (min_gold, max_gold)
}

pub fn solve(boss_file: Option<&str>, player_file: Option<&str>) -> AocResult {
    let path = "input/21/";
    let mut boss_file_name = String::from(path);
    match boss_file {
        Some(name) => boss_file_name.push_str(name),
        None => {
            let name = common::get_input("Boss file name", "boss");
            boss_file_name.push_str(&name);
        }
    }
    let boss = load_file(&boss_file_name).unwrap();

    let mut player_file_name = String::from(path);
    match player_file {
        Some(name) => player_file_name.push_str(name),
        None => {
            let name = common::get_input("Player file name", "player");
            player_file_name.push_str(&name);
        }
    }
    let player = load_file(&player_file_name).unwrap();

    let store = get_store();
    let (min, max) = play(&store, &player, &boss);

    AocResult {
        day: 21,
        phase_1: Some(AocResultType::Usize(min)),
        phase_2: Some(AocResultType::Usize(max)),
    }
}
