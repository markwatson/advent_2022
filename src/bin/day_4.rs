use std::fs::read_to_string;

fn range(item: &str) -> (i32, i32) {
    let mut parts = item.split("-");
    let min = parts.next().unwrap().parse::<i32>().unwrap();
    let max = parts.next().unwrap().parse::<i32>().unwrap();
    return (min, max);
}

// Is a contained by b?
fn contained_by(a: (i32, i32), b: (i32, i32)) -> bool {
    return a.0 >= b.0 && a.1 <= b.1;
}

fn overlap(a: (i32, i32), b: (i32, i32)) -> bool {
    return a.0 <= b.1 && a.1 >= b.0;
}

fn main() {
    let input = read_to_string("./data/day_4").expect("Error reading file");

    let mut number_contained = 0;
    let mut number_overlap = 0;
    for line in input.split("\n") {
        println!("Line: {}", line);
        let pair = line.split(",").collect::<Vec<&str>>();
        let a = range(pair[0]);
        let b = range(pair[1]);

        if contained_by(a, b) || contained_by(b, a) {
            println!("Contained");
            number_contained += 1;
        } else {
            println!("Not contained");
        }

        if overlap(a, b) {
            println!("Overlap");
            number_overlap += 1;
        } else {
            println!("No overlap");
        }
    }

    println!("Number contained: {}", number_contained);
    println!("Number overlap: {}", number_overlap);
}
