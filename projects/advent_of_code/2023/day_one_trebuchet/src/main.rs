use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::num::ParseIntError;
use std::path::Path;

fn main() {
    let mut answer: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line_result in lines {
            let line = line_result.unwrap();
            let mut num_vector: Vec<char> = Vec::new();
            for character in line.chars() {
                if let Some(num) = character.to_digit(10) {
                    num_vector.push(character);
                }
            }
            match get_to_add(num_vector) {
                Ok(value) => answer += value,
                Err(why) => eprintln!("{}", why),
            };
        }
    }
    let pt_2_answer = part_two();
    println!("pt 1: ANSWER: {}", answer);
    println!("pt 2: ANSWER: {}", pt_2_answer);
}

fn get_to_add(char_vector: Vec<char>) -> Result<u32, ParseIntError> {
    let concatenated = format!("{}{}", char_vector.first().unwrap(), char_vector.last().unwrap());
    concatenated.parse::<u32>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename).expect("Could not open file");
    Ok(io::BufReader::new(file).lines())
}

// Do part two using more than just brute force
// goals:
// use enum and implement methods on that enum
// repeat all methods for the sake of repetition: "Repetition är kunskapens moder"
// only reference the docs and previous written files


#[derive(Debug, PartialEq)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

struct DigitIterator {
    digit: Digit,
    index: u8,
}
impl Iterator for DigitIterator {
    type Item = Digit;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        Digit::from_int(self.index)
    }
}

impl Digit {
    fn to_string(&self) -> String {
        match self {
            Digit::One => String::from("one"),
            Digit::Two => String::from("two"),
            Digit::Three => String::from("three"),
            Digit::Four => String::from("four"),
            Digit::Five => String::from("five"),
            Digit::Six => String::from("six"),
            Digit::Seven => String::from("seven"),
            Digit::Eight => String::from("eight"),
            Digit::Nine => String::from("nine"),
        }
    }
    fn to_int(&self) -> u8 {
        match self {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }

    fn from_string(input: &str) -> Option<Digit> {
        match input.to_lowercase().as_str() {
            "one" => Some(Digit::One),
            "two" => Some(Digit::Two),
            "three" => Some(Digit::Three),
            "four" => Some(Digit::Four),
            "five" => Some(Digit::Five),
            "six" => Some(Digit::Six),
            "seven" => Some(Digit::Seven),
            "eight" => Some(Digit::Eight),
            "nine" => Some(Digit::Nine),
            _ => None,
        }
    }

    fn from_int(integer: u8) -> Option<Digit> {
        match integer {
            1 => Some(Digit::One),
            2 => Some(Digit::Two),
            3 => Some(Digit::Three),
            4 => Some(Digit::Four),
            5 => Some(Digit::Five),
            6 => Some(Digit::Six),
            7 => Some(Digit::Seven),
            8 => Some(Digit::Eight),
            9 => Some(Digit::Nine),
            _ => None,
        }
    }

    fn iter() -> DigitIterator {
        DigitIterator {
            digit: Digit::One,
            index: 0,
        }
    }

    fn parse_occurrences(input: &str) -> Vec<Digit> {
        let mut ocurrences: Vec<(usize, Digit)> = Vec::new();
        let iter = Digit::iter();
        let to_check = input.to_lowercase();
        for digit in iter {
            // check written out "one", "nine", ...
            to_check
                .match_indices(digit.to_string().as_str())
                .for_each(|(bindex, value)| {
                    let idx = ocurrences.partition_point(|(at, val)| at < &bindex);
                    ocurrences.insert(idx, (bindex, Digit::from_int(digit.to_int()).unwrap()));
                });
        };
        // check string digits eg. "1", "9", ...
        to_check.match_indices(char::is_numeric)
            .for_each(|(bindex, value)| {
                let idx = ocurrences.partition_point(|(at, val)| at < &bindex);
                ocurrences.insert(idx, (bindex, Digit::from_int(value.parse().unwrap()).unwrap()));
            });

        ocurrences.iter()
            .map(|(_, value)| Digit::from_int(value.to_int()).unwrap())
            .collect()
    }
}

fn concat_digits(first: &Digit, second: &Digit) -> u32 {
    format!("{}{}", first.to_int(), second.to_int()).parse().unwrap()
}

#[test]
fn test_digit() {

    assert_eq!(Digit::from_string("one").unwrap().to_int(), 1);

    assert_eq!(Digit::Five.to_int(), 5);

    assert_eq!(Digit::Six.to_string(), "six");

    assert_eq!(Digit::parse_occurrences("onetwothreefour"),
               [Digit::One, Digit::Two, Digit::Three, Digit::Four]);
    assert_eq!(Digit::parse_occurrences("eightwothree"),
               [Digit::Eight, Digit::Two, Digit::Three]);
    assert_eq!(Digit::parse_occurrences("1f2"),
               [Digit::One, Digit::Two]);
    assert_eq!(Digit::parse_occurrences("one2three4"),
               [Digit::One, Digit::Two, Digit::Three, Digit::Four]);
}


fn read_lines_simpler(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file to string")
}
fn part_two() -> u32 {
    let input = read_lines_simpler("input_pt2.txt");
    let mut total = 0;
    for line in input.lines() {
        // println!("{},{:?}", line, Digit::parse_occurrences(line));
        let parsed = Digit::parse_occurrences(line);
        total += concat_digits(parsed.first().unwrap(), parsed.last().unwrap());
    }
    total
}
