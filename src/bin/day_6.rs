use std::collections::HashSet;
use std::fs::read_to_string;
use std::str::Chars;

fn find_marker(chars: Chars, marker_len: usize) -> i32 {
    let mut prev: Vec<char> = vec![];
    let mut set_helper = HashSet::with_capacity(100);
    let mut i = 1;
    for c in chars {
        prev.push(c);
        if prev.len() > marker_len {
            prev.remove(0);
        }

        set_helper.clear();
        prev.iter().for_each(|x| {
            set_helper.insert(x.clone());
        });

        if set_helper.len() == marker_len {
            return i;
        }

        i += 1;
    }

    panic!("Marker not found");
}

fn main() {
    let input = read_to_string("./data/day_6").expect("Error reading file");
    let chars = input.trim().chars();
    println!("Part 1: {}", find_marker(chars.clone(), 4));
    println!("Part 2: {}", find_marker(chars.clone(), 14));
}
