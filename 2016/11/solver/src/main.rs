use std::collections::BTreeMap;
use std::ops::Shl;

// Isotope indices
#[allow(non_upper_case_globals)]
const Po: usize = 0; // Polonium
#[allow(non_upper_case_globals)]
const Tm: usize = 1; // Thulium
#[allow(non_upper_case_globals)]
const Pm: usize = 2; // Promethium
#[allow(non_upper_case_globals)]
const Ru: usize = 3; // Ruthenium
#[allow(non_upper_case_globals)]
const Co: usize = 4; // Cobalt
// Part 2
#[allow(non_upper_case_globals)]
const El: usize = 5; // Elerium
#[allow(non_upper_case_globals)]
const Di: usize = 6; // Dilithium

#[allow(non_upper_case_globals)]
const Isotopes: [usize; 7] = [Po, Tm, Pm, Ru, Co, El, Di];
// part 1: const Isotopes: [usize; 5] = [Po, Tm, Pm, Ru, Co];


#[derive(Debug, Clone, Copy)]
struct Floor {
    generators: [bool; 7],
    chips: [bool; 7],
}

impl Floor {
    pub fn new(g: Vec<usize>, c: Vec<usize>) -> Floor {
        let mut f = Floor {
            generators: [false; 7],
            chips: [false; 7],
        };
        for gen in g {
            f.generators[gen] = true;
        }
        for chp in c {
            f.chips[chp] = true;
        }
        f
    }
    pub fn id(&self) -> u16 {
        let mut id: u16 = 0b0000_0000_0000_0000;
        let mut bm: u16 = 0b0000_0000_0000_0001;
        for i in (0..self.generators.len()).rev() {
            if self.chips[i] {
                id = id | bm;
            }
            bm = bm.shl(1);
            if self.generators[i] {
                id = id | bm;
            }
            bm = bm.shl(1);
        }

        id
    }
    pub fn is_valid(&self) -> bool {
        let mut has_generators = false;
        for g in 0..self.generators.len() {
            if self.generators[g] {
                has_generators = true;
            }
        }
        if has_generators {
            for i in 0..self.chips.len() {
                if self.chips[i] && !self.generators[i] {
                    return false;
                }
            }
        }
        true
    }
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} | {} {} {} {} : {}",
            if self.generators[Po] { "PoG" } else { "   " },
            if self.chips[Po] { "PoM" } else { "   " },
            if self.generators[Tm] { "TmG" } else { "   " },
            if self.chips[Tm] { "TmM" } else { "   " },
            if self.generators[Pm] { "PmG" } else { "   " },
            if self.chips[Pm] { "PmM" } else { "   " },
            if self.generators[Ru] { "RuG" } else { "   " },
            if self.chips[Ru] { "RuM" } else { "   " },
            if self.generators[Co] { "CoG" } else { "   " },
            if self.chips[Co] { "CoM" } else { "   " },
            if self.generators[El] { "ElG" } else { "   " },
            if self.chips[El] { "ElM" } else { "   " },
            if self.generators[Di] { "DiG" } else { "   " },
            if self.chips[Di] { "DiM" } else { "   " },
            self.id()
                )
    }
}

impl std::fmt::Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
struct Facility {
    elevator_floor: usize,
    floors: [Floor; 4],
}

impl Facility {
    pub fn new(f4g: Vec<usize>, f4c: Vec<usize>,
               f3g: Vec<usize>, f3c: Vec<usize>,
               f2g: Vec<usize>, f2c: Vec<usize>,
               f1g: Vec<usize>, f1c: Vec<usize>,
               elevator_floor: usize
               ) -> Facility {
        
        let f4 = Floor::new(f4g, f4c);
        let f3 = Floor::new(f3g, f3c);
        let f2 = Floor::new(f2g, f2c);
        let f1 = Floor::new(f1g, f1c);
        
        Facility {
            elevator_floor: elevator_floor,
            floors: [f1, f2, f3, f4],
        }
    }
    pub fn first_state() -> Facility {
        // initial data
        Facility::new(vec![], vec![],
                      vec![], vec![],
                      vec![], vec![Po, Pm],
                      //vec![Po, Tm, Pm, Ru, Co], vec![Tm, Ru, Co],
                      vec![Po, Tm, Pm, Ru, Co, El, Di], vec![Tm, Ru, Co, El, Di],
                      0)
    }
    pub fn final_state() -> Facility {
        // finalA data
        Facility::new(vec![Po, Tm, Pm, Ru, Co, El, Di],
                      vec![Po, Tm, Pm, Ru, Co, El, Di],
                      vec![], vec![],
                      vec![], vec![],
                      vec![], vec![],
                      3)
    }
    pub fn demo_first_state() -> Facility {
        // initial data
        Facility::new(vec![], vec![],
                      vec![Tm], vec![],
                      vec![Po], vec![],
                      vec![], vec![Po,Tm],
                      0)
    }
    pub fn demo_final_state() -> Facility {
        // finalA data
        Facility::new(vec![Po, Tm], vec![Po, Tm],
                      vec![], vec![],
                      vec![], vec![],
                      vec![], vec![],
                      3)
    }
    pub fn is_valid(&self) -> bool {
        self.floors[0].is_valid() &&
            self.floors[1].is_valid() &&
            self.floors[2].is_valid() &&
            self.floors[3].is_valid()
    }
    pub fn id(&self) -> u64 {
        let mut id: u64 = 0;
        id = id | self.floors[0].id() as u64
            | (self.floors[1].id() as u64).shl(16)
            | (self.floors[2].id() as u64).shl(32)
            | (self.floors[3].id() as u64).shl(48)
            | (1 as u64).shl(15 + 16*self.elevator_floor);
        id
    }

    
    pub fn next_states(&self) -> Vec<Facility> {
        let mut fv: Vec<Facility> = Vec::new();
        let e = self.elevator_floor;
        let next_floors: [usize; 2] = [
            if e < self.floors.len() - 1 {
                e+1
            } else { e-1 },
            if e > 0 {
                e-1
            } else { e+1}
            ];
        let handle_floors = if e > 0 && e < self.floors.len()-1 {2} else {1};

        // Move one item up or down
        for fi in 0..handle_floors {
            let nf = next_floors[fi];
            for i in 0..Isotopes.len() {
                // Generators
                if self.floors[e].generators[i] {
                    let mut next = self.clone();
                    next.floors[e].generators[i] = false;
                    next.floors[nf].generators[i] = true;
                    next.elevator_floor = nf;
                    if next.is_valid() {
                        fv.push(next);
                    }
                }
                // Chips
                if self.floors[e].chips[i] {
                    let mut next = self.clone();
                    next.floors[e].chips[i] = false;
                    next.floors[nf].chips[i] = true;
                    next.elevator_floor = nf;
                    if next.is_valid() {
                        fv.push(next);
                    }
                }
            }
        }
        // Move two items up or down
        // Possible combos: G+G, M+M, xG+xM
        for fi in 0..handle_floors {
            let nf = next_floors[fi];
            for i in 0..Isotopes.len() {
                // xG + xM
                if self.floors[e].generators[i] && self.floors[e].chips[i] {
                    let mut next = self.clone();
                    next.floors[e].generators[i] = false;
                    next.floors[nf].generators[i] = true;
                    next.floors[e].chips[i] = false;
                    next.floors[nf].chips[i] = true;
                    next.elevator_floor = nf;
                    if next.is_valid() {
                        fv.push(next);
                    }
                }
                // G+G
                if self.floors[e].generators[i] {
                    for j in i+1..Isotopes.len() {
                        if self.floors[e].generators[j] {
                            let mut next = self.clone();
                            next.floors[e].generators[i] = false;
                            next.floors[e].generators[j] = false;
                            next.floors[nf].generators[i] = true;
                            next.floors[nf].generators[j] = true;
                            next.elevator_floor = nf;
                            if next.is_valid() {
                                fv.push(next);
                            }
                        }
                    }
                }
                // M+M
                if self.floors[e].chips[i] {
                    for j in i+1..Isotopes.len() {
                        if self.floors[e].chips[j] {
                            let mut next = self.clone();
                            next.floors[e].chips[i] = false;
                            next.floors[e].chips[j] = false;
                            next.floors[nf].chips[i] = true;
                            next.floors[nf].chips[j] = true;
                            next.elevator_floor = nf;
                            if next.is_valid() {
                                fv.push(next);
                            }
                        }
                    }
                }
            }
        }
        fv
    }
}

impl std::fmt::Display for Facility {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f
               , "F4: {} {}\nF3: {} {}\nF2: {} {}\nF1: {} {}\nID: {}"
               , if self.elevator_floor == 3 { "E" } else { " " }
               , self.floors[3].to_string()
               , if self.elevator_floor == 2 { "E" } else { " " }
               , self.floors[2].to_string()
               , if self.elevator_floor == 1 { "E" } else { " " }
               , self.floors[1].to_string()
               , if self.elevator_floor == 0 { "E" } else { " " }
               , self.floors[0].to_string()
               , self.id()
               )
    }
}

fn main() {
    println!("AOC 11");

    let start = Facility::first_state();
    let stop = Facility::final_state();
    
    println!("Start from: \n{}", start);
    println!("Is valid: {}, ID: {}\n{:064b}",
             start.is_valid(), start.id(), start.id());

    println!("Go to: \n{}", stop);
    println!("Is valid: {}, ID: {}\n{:064b}",
             stop.is_valid(), stop.id(), stop.id());

    let mut all_states: BTreeMap<u64, Facility> = BTreeMap::new();
    let mut unhandled: Vec<Facility> = Vec::with_capacity(15);
    let mut level = 0;
    unhandled.push(start);
    all_states.insert(start.id(), start);
    
    'outer: loop {
        level += 1;
        let processing: Vec<_> = unhandled.drain(0..).collect();
        for f in processing {
            let new_states = f.next_states();
            for ns in new_states {
                if ns.id() == stop.id() {
                    println!("Reached stop at level {}, {} nodes!", level, all_states.len());
                    break 'outer;
                }
                if ! all_states.contains_key(&ns.id()) {
                    all_states.insert(ns.id(), ns);
                    unhandled.push(ns);
                }
            }
        }
    }

}
