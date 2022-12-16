use serde_json::Value;
use std::{cmp, fs::read_to_string};

fn parse(fname: &str) -> Vec<Vec<Value>> {
    let lines = read_to_string(fname).expect("Error reading file");

    let mut pairs = vec![];
    let mut pair = vec![];
    for line in lines.split('\n') {
        if line.trim() == "" {
            pairs.push(pair);
            pair = vec![];
            continue;
        }

        let v: Value = serde_json::from_str(line).unwrap();
        pair.push(v);
    }
    pairs.push(pair);
    return pairs;
}

fn correct_order(left: &Vec<Value>, right: &Vec<Value>, nesting: i32) -> Option<bool> {
    if nesting == 0 {
        println!(
            "- {} <> {}",
            serde_json::to_string(left).unwrap(),
            serde_json::to_string(right).unwrap(),
        );
    }

    let min_len = cmp::min(left.len(), right.len());
    for i in 0..min_len {
        let l = &left[i];
        let r = &right[i];
        let prefix = (0..nesting).map(|_| " ").collect::<String>();
        println!(
            "{}  - {} <> {}",
            prefix,
            serde_json::to_string(l).unwrap(),
            serde_json::to_string(r).unwrap(),
        );

        if l.is_number() && r.is_number() {
            if l.as_i64() < r.as_i64() {
                return Some(true);
            } else if l.as_i64() > r.as_i64() {
                return Some(false);
            } else {
                continue;
            }
        } else if l.is_array() && r.is_number() {
            return correct_order(
                l.as_array().unwrap(),
                &vec![Value::from(r.as_i64().unwrap())],
                nesting + 2,
            );
        } else if l.is_number() && r.is_array() {
            return correct_order(
                &vec![Value::from(l.as_i64().unwrap())],
                r.as_array().unwrap(),
                nesting + 2,
            );
        } else if l.is_array() && r.is_array() {
            let result = correct_order(l.as_array().unwrap(), r.as_array().unwrap(), nesting + 2);
            if result.is_none() {
                continue;
            } else {
                return result;
            }
        } else {
            panic!("BAD TYPES");
        }
    }
    if left.len() > right.len() {
        // right ran out first
        println!("    -> right ran out first");
        return Some(false);
    } else if left.len() < right.len() {
        // left ran out first
        println!("    -> left ran out first");
        return Some(true);
    } else {
        println!("    -> same length");
        return None;
    }
}

fn main() {
    let pairs = parse("./data/day_13");

    let mut i = 1;
    let mut correct = 0;
    for pair in pairs {
        assert!(pair.len() == 2);
        let left = &pair[0];
        let right = &pair[1];
        println!("\nComparing pair {}", i);
        let result = correct_order(left.as_array().unwrap(), right.as_array().unwrap(), 0);
        match result {
            Some(true) => {
                println!("Correct order!");
                correct += i;
            }
            Some(false) => println!("Incorrect order"),
            None => panic!("No order found!"),
        }
        i += 1;
    }

    println!("\nStep 1: sum of correct indexes: {}", correct);
}
