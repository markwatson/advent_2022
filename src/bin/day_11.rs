use std::{cell::RefCell, collections::HashMap, fs::read_to_string, process::exit};

use num_bigint::{BigInt, BigUint};
use regex::Regex;

#[derive(Debug, Clone)]
struct Item {
    start: u64,
    running: u64,
    factors: Vec<u64>,
}

impl Item {
    fn new(start: u64) -> Self {
        Item {
            start,
            running: start,
            factors: primes::factors(start),
        }
    }

    fn clone(&self) -> Self {
        Item {
            start: self.start.clone(),
            running: self.running.clone(),
            factors: self.factors.clone(),
        }
    }

    fn calc_from_factors(&self) -> u64 {
        let mut new_number: u64 = 1;
        for factor in self.factors.iter() {
            new_number = new_number.checked_mul(*factor).expect("INTEGER OVERFLOW");
        }
        return new_number;
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: RefCell<Vec<u64>>,
    items_factors: RefCell<Vec<Item>>,
    operation: (String, Option<String>, Option<u64>),
    test: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: RefCell::new(vec![]),
            items_factors: RefCell::new(vec![]),
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
    fn perform_operation(&self, item: u64) -> u64 {
        let mut new = 0;
        if self.operation.1.is_some() {
            if self.operation.0 == "*" {
                new = item.pow(2);
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
        return new;
    }

    fn perform_operation_primes(&self, item: &mut Item) {
        // println!(
        //     "Prime factors: {}={:?}",
        //     item.start,
        //     prime_factors_sieve(item.start)
        // );
        println!("Performing operation on {:?}", item);
        println!("operation: {:?}", self.operation);
        let mut new_factors = item.factors.clone();
        if self.operation.1.is_some() {
            if self.operation.0 == "*" {
                new_factors.extend(item.factors.iter());
            } else if self.operation.0 == "+" {
                new_factors.push(2);
            }
        } else if self.operation.2.is_some() {
            if self.operation.0 == "*" {
                new_factors.push(self.operation.2.unwrap());
            } else if self.operation.0 == "+" {
                // TODO: This can't work?
                // Adding numbers causes us to have to recalc factors :/
                let mut new_number = item.calc_from_factors();
                new_number += self.operation.2.unwrap();
                new_factors = primes::factors(new_number);
            }
        }
        item.factors = new_factors;
        // item.running = self.perform_operation(item.running);
        println!("Result: {:?}", item);
        // if item.running != item.calc_from_factors() {
        //     panic!("BAD THING HAPPENED.")
        // };
    }

    fn play_round(monkeys: &Vec<Monkey>, inspected: &mut HashMap<usize, u64>, divide_by_3: bool) {
        println!("=======");
        for monkey in monkeys.iter() {
            println!();
            if divide_by_3 {
                let num_items = monkey.items.borrow().len();
                for _ in 0..num_items {
                    println!();
                    *inspected.entry(monkey.id).or_insert(0) += 1;
                    let item = monkey.items.borrow_mut().remove(0);
                    let mut new_item = monkey.perform_operation(item);
                    if divide_by_3 {
                        new_item = new_item / 3;
                    }

                    if &new_item % monkey.test == 0 {
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
            } else {
                println!("Working on monkey: {:?}", monkey.id);
                let num_items = monkey.items_factors.borrow().len();
                for _ in 0..num_items {
                    *inspected.entry(monkey.id).or_insert(0) += 1;

                    let item = monkey.items_factors.borrow_mut().remove(0);
                    // TODO: So many copies?
                    let mut item_new = item.clone();
                    monkey.perform_operation_primes(&mut item_new);

                    // Test is always prime.
                    println!(
                        "Testing factors div by {}: {:?}",
                        &monkey.test, item_new.factors
                    );

                    let mut send_to_monkey = monkey.false_monkey;
                    for f in &item_new.factors {
                        if f == &monkey.test {
                            send_to_monkey = monkey.true_monkey;
                            break;
                        }
                    }
                    // if item_new.running % monkey.test == 0 {
                    //     send_to_monkey = monkey.true_monkey;
                    // }

                    println!("Sending to monkey: {:?}", send_to_monkey);
                    monkeys[send_to_monkey]
                        .items_factors
                        .borrow_mut()
                        .push(item_new);
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
                    .map(|item| item.trim().parse::<u64>().unwrap())
                    .collect(),
            );
            monkey.items_factors = RefCell::new(
                items[1]
                    .split(',')
                    .map(|item| Item::new(item.trim().parse::<u64>().unwrap()))
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
                (o, v) => (o.to_string(), None, Some(v.parse::<u64>().unwrap())),
            };
        });
        if operation.is_some() {
            continue;
        }

        // Test
        let test = re_test.captures(line).map(|operation| {
            monkey.test = operation[1].trim().parse::<u64>().unwrap();
            assert!(primes::is_prime(monkey.test));
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

fn monkey_around(monkey_def: &str, rounds: u64, divide_by_3: bool) -> u64 {
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
    let mut m = inspected.values().cloned().collect::<Vec<u64>>();
    m.sort();
    let monkey_business = m[m.len() - 1] * m[m.len() - 2];
    return monkey_business;
}

fn main() {
    let monkey_business = monkey_around("./data/day_11", 20, true);
    println!("Step 1: Monkey business: {}", monkey_business);

    let monkey_business = monkey_around("./data/day_11_test", 20, false);
    println!("Step 2: Monkey business: {}", monkey_business);
}
