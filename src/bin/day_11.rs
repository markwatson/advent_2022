use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: (String, Option<String>, Option<i32>),
    test: i32,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: vec![],
            operation: (String::new(), None, None),
            test: 0,
            true_monkey: 0,
            false_monkey: 0,
        }
    }
}

fn parse_monkeys(fname: &str) -> Vec<Monkey> {
    let re_monkey = Regex::new(r"^Monkey (\d+):$").unwrap();
    let re_items = Regex::new(r"^[ ]+Starting items: ([0-9, ]+)$").unwrap();
    let re_operation = Regex::new(r"^[ ]+Operation: new = old ([\+\*]) (.+)$").unwrap();
    let re_test = Regex::new(r"^[ ]+Test: divisible by (\d+)$").unwrap();
    let re_if_true = Regex::new(r"^[ ]+If true: throw to monkey (\d+)$").unwrap();
    let re_if_false = Regex::new(r"^[ ]+If false: throw to monkey (\d+)$").unwrap();

    let input = read_to_string(fname).expect("Error reading file");

    let mut output = vec![];
    let mut monkey = Monkey::new();
    for line in input.split('\n') {
        if line.trim().is_empty() {
            output.push(monkey);
            monkey = Monkey::new();
        }

        // Parse the monkey
        // ID
        let id = re_monkey.captures(line).map(|id| {
            monkey.id = id[1].parse::<usize>().unwrap();
        });
        if id.is_some() {
            continue;
        }

        // Items
        let items = re_items.captures(line).map(|items| {
            monkey.items = items[1]
                .split(',')
                .map(|item| item.trim().parse::<i32>().unwrap())
                .collect();
        });
        if items.is_some() {
            continue;
        }

        // Operation
        let operation = re_operation.captures(line).map(|operation| {
            let op = operation[1].trim();
            let val = operation[2].trim();

            monkey.operation = match (op, val) {
                (o, "old") => (o.to_string(), Some("old".to_string()), None),
                (o, v) => (o.to_string(), None, Some(v.parse::<i32>().unwrap())),
            };
        });
        if operation.is_some() {
            continue;
        }

        // Test
        let test = re_test.captures(line).map(|operation| {
            monkey.test = operation[1].trim().parse::<i32>().unwrap();
        });
        if test.is_some() {
            continue;
        }

        // If True
        let if_true = re_if_true.captures(line).map(|operation| {
            monkey.true_monkey = operation[1].parse::<usize>().unwrap();
        });
        if if_true.is_some() {
            continue;
        }

        // If False
        let if_false = re_if_false.captures(line).map(|operation| {
            monkey.false_monkey = operation[1].parse::<usize>().unwrap();
        });
        if if_false.is_some() {
            continue;
        }
    }
    output.push(monkey);
    return output;
}

fn main() {
    let monkeys = parse_monkeys("./data/day_11");
    for m in monkeys {
        println!("{:?}", m);
    }
}
