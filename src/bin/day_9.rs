use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
struct Instruction {
    direction: (i32, i32),
    distance: i32,
}

impl Instruction {
    fn new(direction: char, distance: i32) -> Instruction {
        Instruction {
            direction: match direction {
                'U' => (0, 1),
                'D' => (0, -1),
                'L' => (-1, 0),
                'R' => (1, 0),
                _ => panic!("Invalid direction"),
            },
            distance,
        }
    }

    fn read_from(fname: &str) -> Vec<Instruction> {
        let input = read_to_string(fname).expect("Error reading file");

        let mut output = vec![];
        for line in input.lines() {
            let parts: Vec<&str> = line.trim().split(' ').collect();
            if parts.len() != 2 {
                panic!("Invalid input");
            }
            let direction = parts[0].chars().next().unwrap();
            let distance = parts[1].parse::<i32>().unwrap();
            output.push(Instruction::new(direction, distance));
        }
        output
    }
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn abs_sub(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    ((a.0 - b.0).abs(), (a.1 - b.1).abs())
}
fn next_to(h: (i32, i32), t: (i32, i32)) -> bool {
    let diff = abs_sub(h, t);
    h == t || (diff.0 <= 1 && diff.1 <= 1)
}

fn direction_to_move(h: (i32, i32), t: (i32, i32)) -> (i32, i32) {
    if next_to(h, t) {
        // Don't move
        (0, 0)
    } else if h.0 == t.0 && h.1 > t.1 {
        // Same column, head is above tail
        (0, 1)
    } else if h.0 == t.0 && h.1 < t.1 {
        // Same column, head is below tail
        (0, -1)
    } else if h.1 == t.1 && h.0 > t.0 {
        // Same row, head is to the right of tail
        (1, 0)
    } else if h.1 == t.1 && h.0 < t.0 {
        // Same row, head is to the left of tail
        (-1, 0)
    } else if h.1 > t.1 && h.0 > t.0 {
        // Head is above and to the right of tail
        (1, 1)
    } else if h.1 > t.1 && h.0 < t.0 {
        // Head is above and to the left of tail
        (-1, 1)
    } else if h.1 < t.1 && h.0 > t.0 {
        // Head is below and to the right of tail
        (1, -1)
    } else if h.1 < t.1 && h.0 < t.0 {
        // Head is below and to the left of tail
        (-1, -1)
    } else {
        panic!("I'm lost!");
    }
}

fn simulate(instructions: Vec<Instruction>) -> (i32, i32) {
    let step_2_tail_length = 9;
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    let start = (0, 0);
    let mut head = start;
    let mut tail = start;

    let mut tails = vec![start; step_2_tail_length + 1];
    let mut visited_long: HashMap<(i32, i32), bool> = HashMap::new();

    for command in instructions {
        for _ in 0..command.distance {
            // Step 1
            head = add(head, command.direction);
            let tail_direction = direction_to_move(head, tail);
            tail = add(tail, tail_direction);
            visited.insert(tail, true);

            // Step 2
            tails[0] = head; // track head
            for i in 1..tails.len() {
                let tail_direction = direction_to_move(tails[i - 1], tails[i]);
                tails[i] = add(tails[i], tail_direction);
            }
            visited_long.insert(tails[tails.len() - 1], true);
        }
    }

    (visited.len() as i32, visited_long.len() as i32)
}

fn main() {
    let instructions = Instruction::read_from("./data/day_9");

    let tail_visited = simulate(instructions);
    println!("Step1: Tail visited {} cells", tail_visited.0);
    println!("Step2: LOOONG tail visited {} cells", tail_visited.1);
    // Done!
}
