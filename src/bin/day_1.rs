use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

fn main() {
    let mut elves: Vec<Elf> = parse("./data/day_1");
    if elves.len() == 0 {
        println!("Error reading file");
        exit(1);
    }

    // Find main elf
    let top_elf = top_n_calories(&mut elves, 1);
    println!("Elf with most calories has this many calories: {}", top_elf);

    // Find top three elves
    let top_three = top_n_calories(&mut elves, 3);
    println!("Top three elves have this many calories: {}", top_three);
}

fn top_n_calories(elves: &mut Vec<Elf>, n: usize) -> i32 {
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    return elves.iter().take(n).map(|a: &Elf| a.calories).sum();
}

struct Elf {
    //id: i32,
    calories: i32,
    //items: Vec<i32>,
}

fn parse<P>(filename: P) -> Vec<Elf>
where
    P: AsRef<Path>,
{
    let mut elves: Vec<Elf> = Vec::new();
    let file = File::open(filename);
    if file.is_ok() {
        let lines = BufReader::new(file.unwrap()).lines();

        // Consumes the iterator, returns an (Optional) String
        let mut items = Vec::new();
        //let mut counter: i32 = 1;
        for line in lines {
            if let Ok(item) = line {
                if item.len() > 0 {
                    items.push(item.parse::<i32>().unwrap());
                } else {
                    elves.push(Elf {
                        //id: counter,
                        calories: items.iter().sum(),
                        //items: items.clone(),
                    });
                    items = Vec::new();
                    //counter += 1;
                }
            }
        }
    }
    return elves;
}
