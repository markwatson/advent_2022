use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elves: Vec<Elf> = Vec::new();

    // Parse input
    if let Ok(lines) = read_lines("./data/day_1") {
        // Consumes the iterator, returns an (Optional) String
        let mut items = Vec::new();
        let mut counter: i32 = 1;
        for line in lines {
            if let Ok(item) = line {
                if item.len() > 0 {
                    items.push(item.parse::<i32>().unwrap());
                } else {
                    elves.push(Elf {
                        id: counter,
                        calories: items.iter().sum(),
                        items: items.clone(),
                    });
                    items = Vec::new();
                    counter += 1;
                }
            }
        }
    }

    // Find main elf
    let mut max: i32 = 0;
    for elf in &elves {
        if elf.calories > max {
            max = elf.calories;
        }
    }
    println!("Elf with most calories has this many calories: {}", max);

    // Find top three elves
    let mut top_3_calories: i32 = 0;
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    for elf in elves.iter().take(3) {
        println!("Elf {} has {} calories", elf.id, elf.calories);
        top_3_calories += elf.calories;
    }
    println!("Top 3 elves have this many calories: {}", top_3_calories);
}

struct Elf {
    id: i32,
    calories: i32,
    items: Vec<i32>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
