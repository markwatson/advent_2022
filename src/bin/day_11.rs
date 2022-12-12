use std::{cell::RefCell, collections::HashMap, fs::read_to_string};

use regex::Regex;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: RefCell<Vec<u128>>,
    operation: (String, Option<String>, Option<u128>),
    test: u128,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: RefCell::new(vec![]),
            operation: (String::new(), None, None),
            test: 0,
            true_monkey: 0,
            false_monkey: 0,
        }
    }

    fn items_str(&self) -> String {
        return self
            .items
            .borrow()
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");
    }

    #[allow(dead_code)]
    fn perform_operation(&self, item: u128) -> u128 {
        println!("Before: {}", item);
        let mut new = 0;
        if self.operation.1.is_some() {
            if self.operation.0 == "*" {
                new = item * item;
            } else if self.operation.0 == "+" {
                new = item * 2;
            }
        } else if self.operation.2.is_some() {
            if self.operation.0 == "*" {
                new = item * self.operation.2.unwrap();
            } else if self.operation.0 == "+" {
                new = item + self.operation.2.unwrap();
            }
        }
        if new == 0 {
            panic!("BAD THING HAPPENED.")
        };
        println!("After: {}", new);
        return new;
    }

    fn play_round(monkeys: &Vec<Monkey>, inspected: &mut HashMap<usize, u128>, divide_by_3: bool) {
        for monkey in monkeys.iter() {
            let num_items = monkey.items.borrow().len();
            for _ in 0..num_items {
                *inspected.entry(monkey.id).or_insert(0) += 1;
                let item = monkey.items.borrow_mut().remove(0);
                let mut new_item = monkey.perform_operation(item);
                if divide_by_3 {
                    new_item = new_item / 3;
                }

                // TODO If double divisible by any of the tests, then remove one factor
                // TODO Fix the item * item
                // let other_primes = vec![2, 3, 5, 7, 11, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71];
                // for p in other_primes {
                //     if new_item % p == 0 {
                //         new_item = new_item / p;
                //     }
                // }
                // LCM somehow?

                let max: u128 = monkeys.iter().map(|f| f.test).product();
                new_item = new_item % max;
                if &new_item % monkey.test == 0 {
                    //new_item = &new_item / monkey.test;
                    monkeys[monkey.true_monkey]
                        .items
                        .borrow_mut()
                        .push(new_item);
                } else {
                    monkeys[monkey.false_monkey]
                        .items
                        .borrow_mut()
                        .push(new_item);
                }
            }
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
            monkey.items = RefCell::new(
                items[1]
                    .split(',')
                    .map(|item| item.trim().parse::<u128>().unwrap())
                    .collect(),
            );
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
                (o, v) => (o.to_string(), None, Some(v.parse::<u128>().unwrap())),
            };
        });
        if operation.is_some() {
            continue;
        }

        // Test
        let test = re_test.captures(line).map(|operation| {
            monkey.test = operation[1].trim().parse::<u128>().unwrap();
            // assert!(primes::is_prime(monkey.test));
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

fn monkey_around(monkey_def: &str, rounds: u128, divide_by_3: bool) -> u128 {
    let debug = false; //rounds < 30;
    let monkeys = parse_monkeys(monkey_def);
    println!("Let's stop monkeying around and calculate some shit!");
    for m in monkeys.iter() {
        println!("{:?}", m);
    }

    let mut inspected = HashMap::new();
    for round in 1..=rounds {
        println!("Round {}!", round);
        if debug {
            println!(
                "After round {}, the monkeys are holding items with these worry levels:",
                round
            );
        }
        Monkey::play_round(&monkeys, &mut inspected, divide_by_3);
        if debug {
            for m in monkeys.iter() {
                println!("Monkey {}: {}", m.id, m.items_str());
            }
        }
    }

    for m in monkeys.iter() {
        println!(
            "Monkey {} inspected items {} times.",
            m.id, inspected[&m.id]
        );
    }
    let mut m = inspected.values().cloned().collect::<Vec<u128>>();
    m.sort();
    let monkey_business = m[m.len() - 1] * m[m.len() - 2];
    return monkey_business;
}

fn main() {
    // let monkey_business = monkey_around("./data/day_11", 20, true);
    // println!("Step 1: Monkey business: {}", monkey_business);

    let monkey_business = monkey_around("./data/day_11", 10_000, false);
    println!("Step 2: Monkey business: {}", monkey_business);
}
