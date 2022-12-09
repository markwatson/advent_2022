use std::fs::read_to_string;

fn read_vec(fname: &str) -> Vec<Vec<u8>> {
    let input = read_to_string(fname).expect("Error reading file");
    let mut vec = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for c in line.trim().chars() {
            line_vec.push(c.to_digit(10).unwrap() as u8);
        }
        vec.push(line_vec);
    }
    return vec;
}

fn main() {
    let input = read_vec("./data/day_8");

    let mut max_scenic = 0;
    let mut visible = (input.len() * 2) + ((input[0].len() - 2) * 2);
    println!("Step 1: visible = {}", visible);
    for y in 1..input.len() - 1 {
        for x in 1..input[y].len() - 1 {
            let height = input[y][x];

            // Check left
            let mut visible_left = true;
            let mut left = x;
            let mut left_scenic = 0;
            while left > 0 {
                left -= 1;
                left_scenic += 1;
                if input[y][left] >= height {
                    visible_left = false;
                    break;
                }
            }

            // Check right
            let mut visible_right = true;
            let mut right = x;
            let mut right_scenic = 0;
            while right < input[y].len() - 1 {
                right += 1;
                right_scenic += 1;
                if input[y][right] >= height {
                    visible_right = false;
                    break;
                }
            }

            // Check down
            let mut visible_down = true;
            let mut down = y;
            let mut down_scenic = 0;
            while down < input.len() - 1 {
                down += 1;
                down_scenic += 1;
                if input[down][x] >= height {
                    visible_down = false;
                    break;
                }
            }

            // Check up
            let mut visible_up = true;
            let mut up = y;
            let mut up_scenic = 0;
            while up > 0 {
                up -= 1;
                up_scenic += 1;
                if input[up][x] >= height {
                    visible_up = false;
                    break;
                }
            }

            let scenic = left_scenic * right_scenic * down_scenic * up_scenic;
            if scenic > max_scenic {
                max_scenic = scenic;
            }

            // Check visibility
            if visible_left || visible_right || visible_down || visible_up {
                //println!("({}, {}) = {} is visible", x, y, height);
                visible += 1;
            } else {
                //println!("({}, {}) = {} not visible", x, y, height);
            }
        }
    }

    println!("\n Step 1: visible = {}", visible);
    println!("Step 2: max_scenic = {}", max_scenic);
}
