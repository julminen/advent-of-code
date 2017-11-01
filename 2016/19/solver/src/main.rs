use std::env;
use std::time::Instant;

#[derive(Debug)]
struct Elf {
    id: usize,
    presents: usize,
    steal_from: usize,
    next: usize,
    previous: usize,
}

impl Elf {
    fn new(
        id: usize,
        presents: usize,
        steal_from: usize,
        next:usize,
        previous: usize) -> Elf
    {
        Elf {
            id: id,
            presents: presents,
            steal_from: steal_from,
            next: next,
            previous: previous,
        }
    }
}

fn get_next(current_elf: usize, count: usize, elves: &Vec<Elf>) -> usize {
    let mut next = current_elf;
    for _ in 0..count {
        next = elves[next].next;
    }
    next
}

fn main() {
    println!("AOC 19");

    let default_input = 3005290;

    let args: Vec<String> = env::args().collect();
    let elf_count: usize =
        if args.len() > 1 {
            args[1].parse().unwrap()
        } else {
            println!("No input given, using default");
            default_input
        };

    println!("Elves: {}", elf_count);

    let mut elves: Vec<Elf> = Vec::with_capacity(elf_count);
    for i in 0..elf_count {
        elves.push(
            Elf::new(
                i+1,
                1,
                (i+1) % elf_count,
                (i+1) % elf_count,
                (i + elf_count - 1) % elf_count,
                )
                );
    }

    // Steal from next
    println!("Steal from next");
    let mut now = Instant::now();
    let mut current_elf: usize = 0;
    'outer: loop {
        let other_elf = elves[current_elf].steal_from;
        if elves[other_elf].presents == 0 {
            println!(":( {:?} -> {:?}", elves[current_elf], elves[other_elf]);
            break;
        }

        elves[current_elf].presents += elves[other_elf].presents;
        elves[other_elf].presents = 0;
        elves[current_elf].steal_from = elves[other_elf].steal_from;
        
        if elves[current_elf].steal_from == current_elf {
            print!("Finished steal from next. ");
            println!("Remaining elf: {} with {} presents",
                     elves[current_elf].id, elves[current_elf].presents);
            let dur = now.elapsed();
            println!("Tool {}.{:09} sec", dur.as_secs(), dur.subsec_nanos());
            break;
        }
        current_elf = elves[other_elf].steal_from;
    }

    
    // Steal from opposite
    println!("\nSteal from opposite");
    now = Instant::now();
    for (i, e) in elves.iter_mut().enumerate() {
        e.presents = 1;
        e.steal_from = (i + elf_count / 2) % elf_count;
    }

    current_elf = 0;
    let mut other_elf = elf_count / 2;
    let mut active_elf_count = elf_count;
    let mut opposite_incr = if elf_count % 2 == 0 { 1 } else { 2 };
    
    loop {
        elves[current_elf].steal_from = other_elf;

        if active_elf_count == 1 {
            print!("Finished steal from opposite. ");
            println!("Remaining elf: {} with {} presents",
                     elves[current_elf].id, elves[current_elf].presents);
            let dur = now.elapsed();
            println!("Tool {}.{:09} sec", dur.as_secs(), dur.subsec_nanos());
            break;
        }

        elves[current_elf].presents += elves[other_elf].presents;
        elves[other_elf].presents = 0;
        let previous = elves[other_elf].previous;
        let next = elves[other_elf].next;
        elves[previous].next = next;
        elves[next].previous = previous;
        active_elf_count -= 1;
        
        current_elf = elves[current_elf].next;

        // next opposite
        other_elf = get_next(other_elf, opposite_incr, &elves);
        opposite_incr = if opposite_incr == 2 { 1 } else { 2 };
    }
    
}
