use std::fs::read_to_string;

#[derive(PartialEq, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_char(c: char) -> Option<Outcome> {
        match c {
            'X' => Some(Outcome::Lose),
            'Y' => Some(Outcome::Draw),
            'Z' => Some(Outcome::Win),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i64 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn beats(&self, other: &Shape) -> bool {
        match *self {
            Shape::Rock => match *other {
                Shape::Scissors => true,
                _ => false,
            },
            Shape::Paper => match *other {
                Shape::Rock => true,
                _ => false,
            },
            Shape::Scissors => match *other {
                Shape::Paper => true,
                _ => false,
            },
        }
    }

    fn from_char(c: char) -> Option<Shape> {
        match c {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            'X' => Some(Shape::Rock),
            'Y' => Some(Shape::Paper),
            'Z' => Some(Shape::Scissors),
            _ => None,
        }
    }

    fn play(opponent: Shape, you: Shape) -> i64 {
        let score = if you.beats(&opponent) {
            6 // Win - 6 points
        } else if you == opponent {
            3 // Draw - 3 points
        } else {
            0 // Lose - 0 points
        };

        you.score() + score
    }

    fn play_chars(opponent: char, you: char) -> i64 {
        let their_shape = Shape::from_char(opponent).expect("Invalid opponent shape");
        let your_shape = Shape::from_char(you).expect("Invalid your shape");

        Shape::play(their_shape, your_shape)
    }

    fn play_with_goal(opponent: char, outcome: char) -> i64 {
        let outcome = Outcome::from_char(outcome).expect("Invalid outcome");
        let their_shape = Shape::from_char(opponent).expect("Invalid opponent shape");

        let our_shape = match (outcome, their_shape) {
            (Outcome::Lose, Shape::Rock) => Shape::Scissors,
            (Outcome::Lose, Shape::Paper) => Shape::Rock,
            (Outcome::Lose, Shape::Scissors) => Shape::Paper,
            (Outcome::Draw, shape) => shape,
            (Outcome::Win, Shape::Rock) => Shape::Paper,
            (Outcome::Win, Shape::Paper) => Shape::Scissors,
            (Outcome::Win, Shape::Scissors) => Shape::Rock,
        };

        Shape::play(their_shape, our_shape)
    }
}

fn main() {
    let input = read_to_string("./data/day_2").expect("Error reading file");

    let mut score = 0;
    let mut score_with_goal = 0;
    for line in input.split('\n') {
        let chars = line.chars().collect::<Vec<char>>();

        if chars.len() < 3 {
            panic!("Invalid input");
        }

        let theirs = chars[0];
        let yours = chars[2];

        score += Shape::play_chars(theirs, yours);
        score_with_goal += Shape::play_with_goal(theirs, yours);
    }

    println!("Final score for step 1: {}", score);
    println!("Final score for step 2: {}", score_with_goal);
}
