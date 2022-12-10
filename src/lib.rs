use std::fs::read_to_string;

use regex::Regex;

pub mod primes;
pub mod tree;

pub fn read_in(fname: &str, re: &str) -> Vec<Vec<String>> {
    let re_matcher = Regex::new(re).unwrap();
    let input = read_to_string(fname).expect("Error reading file");

    let mut output = vec![];
    for line in input.split('\n') {
        let parts = re_matcher.captures(line.trim());
        parts.map(|parts| {
            let mut output_parts = vec![];
            for i in 1..parts.len() {
                output_parts.push(parts[i].to_string());
            }
            output.push(output_parts);
        });
    }
    return output;
}

pub fn read_in_map<T>(fname: &str, re: &str, transform: fn(&Vec<String>) -> T) -> Vec<T> {
    let data = read_in(fname, re);
    return data.iter().map(transform).collect();
}
