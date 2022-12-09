use std::{collections::HashSet, fs::read_to_string};

// RUST_LOG=debug cargo run --bin day_3

fn priority(c: char) -> i64 {
    if ('A'..='Z').contains(&c) {
        (c as i64) - ('A' as i64) + 27
    } else if ('a'..='z').contains(&c) {
        (c as i64) - ('a' as i64) + 1
    } else {
        panic!("Invalid character");
    }
}

fn to_set(items: &str) -> HashSet<char> {
    return items.chars().collect::<HashSet<char>>();
}

fn main() {
    env_logger::init();

    let input = read_to_string("./data/day_3").expect("Error reading file");

    let mut group: Vec<&str> = Vec::new();
    let mut group_priority_sum = 0;
    let mut priority_sum = 0;
    for line in input.split('\n') {
        // Step 1
        log::debug!("Line: {}", line);

        let items = line.trim();

        let box_size = items.len() / 2;
        let first = to_set(&items[0..box_size]);
        let second = to_set(&items[box_size..]);
        log::debug!("First: {:?}", first);
        log::debug!("Second: {:?}", second);

        let common = first.intersection(&second).collect::<Vec<&char>>();
        log::debug!("Common: {:?}", common);

        priority_sum += common.iter().map(|c| priority(**c)).sum::<i64>();
        log::debug!("Priority sum: {}", priority_sum);
        log::debug!("\n");

        // Step 2
        group.push(items);
        if group.len() == 3 {
            let first = to_set(group[0]);
            let second = to_set(group[1]);
            let third = to_set(group[2]);
            for c in first {
                if second.contains(&c) && third.contains(&c) {
                    group_priority_sum += priority(c);
                }
            }

            group.clear();
        }
    }

    println!("Priority sum: {}", priority_sum);
    println!("Group priority sum: {}", group_priority_sum);
}
