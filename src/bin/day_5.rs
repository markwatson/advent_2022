use regex::Regex;
use std::fs::read_to_string;

struct MoveCommand {
    from: usize,
    to: usize,
    num_items: usize,
}

fn read_stacks(input: &str) -> Vec<Vec<char>> {
    let re_index_line = Regex::new(r"^[0-9 ]+$").unwrap();

    let mut parsing_stacks = true;
    let mut stack_lines: Vec<&str> = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() || re_index_line.is_match(line) {
            parsing_stacks = false;
            continue;
        }

        if parsing_stacks {
            stack_lines.push(line);
        }
    }

    stack_lines.reverse();
    //println!("Stack lines: {:?}", stack_lines);

    let mut output: Vec<Vec<char>> = Vec::new();

    let num_stacks = stack_lines[0].len() / 4 + 1; // hack
    for _ in 0..num_stacks {
        output.push(Vec::new());
    }

    for line in stack_lines {
        let mut i = 1;
        let mut stack_index = 0;
        while i < line.len() {
            let item = line.chars().nth(i).unwrap();
            if item != ' ' {
                output[stack_index].push(item);
            }
            i += 4;
            stack_index += 1;
        }
    }

    println!("Output: {:?}\n\n=====\n", output);

    output
}

fn read_moves(input: &str) -> Vec<MoveCommand> {
    let re_move = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut stack_lines: Vec<MoveCommand> = Vec::new();

    for line in input.split('\n') {
        let parts = re_move.captures(line);
        parts.map(|parts| {
            let num_items = parts[1].parse::<usize>().unwrap();
            let from = parts[2].parse::<usize>().unwrap();
            let to = parts[3].parse::<usize>().unwrap();

            stack_lines.push(MoveCommand {
                from,
                to,
                num_items,
            });
        });
    }

    stack_lines
}

fn main() {
    let input = read_to_string("./data/day_5").expect("Error reading file");

    let mut stacks = read_stacks(&input);
    let moves = read_moves(&input);

    println!("Stacks: {:?}", stacks);
    println!("\n\n");

    // Step 1
    for move_command in moves {
        for _ in 0..move_command.num_items {
            let item = stacks[move_command.from - 1].pop().unwrap();
            stacks[move_command.to - 1].push(item);
        }
    }

    //println!("Stacks: {:?}", stacks);
    //print!("{}", input);

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
    println!("\n STEP 2 \n");

    // Step 2
    let mut stacks = read_stacks(&input);
    let moves = read_moves(&input);

    for move_command in moves {
        let mut temp = vec![];
        for _ in 0..move_command.num_items {
            let item = stacks[move_command.from - 1].pop().unwrap();
            temp.push(item);
        }
        temp.reverse();
        for item in temp {
            stacks[move_command.to - 1].push(item);
        }
    }

    for mut stack in stacks {
        print!("{}", stack.pop().unwrap());
    }
}
