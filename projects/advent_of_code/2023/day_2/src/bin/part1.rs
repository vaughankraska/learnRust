use std::fs;

const RED_THRESH: u8 = 12;
const GREEN_THRESH: u8 = 13;
const BLUE_THRESH: u8 = 14;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_string(input: &str) -> Option<Color> {
        assert!(input.contains("green") || input.contains("blue") || input.contains("red"));
        match input {
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            "red" => Some(Color::Red),
            _ => None
        }
    }
}
#[derive(Debug)]
struct CubePull {
    color: Color,
    count: u32,
}

impl CubePull {
    fn from_string(input: &str) -> CubePull {
        let (count, color) = input.trim().split_once(" ")
            .expect("Did not match format '<number> <color>'");
        CubePull {
            color: Color::from_string(color).unwrap(),
            count: count.parse::<u32>().expect("Cube count could not be parsed"),
        }
    }

    fn is_possible(&self) -> bool {
        match self.color {
            Color::Blue => self.count <= BLUE_THRESH as u32,
            Color::Red => self.count <= RED_THRESH as u32,
            Color::Green => self.count <= GREEN_THRESH as u32,
        }
    }
}

fn main() {
    println!("part one");
    let real_input = fs::read_to_string("src/input_pt1.txt")
        .expect("File issue");
    let answer = part_1(&real_input);
    println!("ANSWER: {}", answer);
}

fn part_1(input: &str) -> u32 {
    let result: Vec<(u32, Vec<Vec<CubePull>>)> = input.lines()
        .map(|game| {
            let (game_str, rounds_string) = game.split_once(":").unwrap();

            let game_number = game_str.split_once(" ").unwrap().1
                .parse::<u32>().unwrap();

            let rounds: Vec<Vec<CubePull>> = rounds_string.split(";")
                .map(|round| {
                    round.trim().split(",")
                        .map(|pull| CubePull::from_string(pull))
                        .collect()
                }).collect();
            (game_number, rounds)
        })
        .collect();
    result.iter().map(|game| {
        for pulls in game.1.iter() {
            for pull in pulls {
                if !pull.is_possible() {
                    return 0;
                }
            }
        }
        game.0
    }).sum()
}

#[cfg(test)]
mod part_1_tests {
    use super::*;

    #[test]
    fn example_solution_matches() {
        let test_input = fs::read_to_string("src/input_pt1_sm.txt")
            .expect("Problem reading input file");
        assert_eq!(part_1(&test_input), 8);
    }
}