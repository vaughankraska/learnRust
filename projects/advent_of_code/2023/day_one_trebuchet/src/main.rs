use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::num::ParseIntError;
use std::path::Path;

fn main() {
    let mut answer: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line_result in lines {
            let line = line_result.unwrap();
            println!();
            let mut num_vector: Vec<char> = Vec::new();
            for character in line.chars() {
                if let Some(num) = character.to_digit(10) {
                    print!("{}", num);
                    num_vector.push(character);
                }
            }
            match get_to_add(num_vector) {
                Ok(value) => answer += value,
                Err(why) => eprintln!("{}", why),
            };
        }
    }
    part_two();
    println!("pt 1: ANSWER: {}", answer);
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
// use struct and implement methods on that struct
// repeat all methods for the sake of repetition: "Repetition är kunskapens moder"
// only reference the docs and previous written files

#[derive(Debug)]
struct Calibration {
    raw_lines: Vec<String>,
    parsed_lines: Vec<Vec<u32>>,
    calibration_values: Vec<u32>,
}
enum WrittenOutNumber {
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
impl WrittenOutNumber {
    fn to_string(&self) -> String {
        match self {
            WrittenOutNumber::One => String::from("one"),
            WrittenOutNumber::Two => String::from("two"),
            WrittenOutNumber::Three => String::from("three"),
            WrittenOutNumber::Four => String::from("four"),
            WrittenOutNumber::Five => String::from("five"),
            WrittenOutNumber::Six => String::from("six"),
            WrittenOutNumber::Seven => String::from("seven"),
            WrittenOutNumber::Eight => String::from("eight"),
            WrittenOutNumber::Nine => String::from("nine"),
        }
    }
    fn match_from_string(input: String) -> WrittenOutNumber {
        match input.to_lowercase() {
            String::from("one") => WrittenOutNumber::One,
            String::from("two") => WrittenOutNumber::Two,
            String::from("three") => WrittenOutNumber::Three,
            String::from("four") => WrittenOutNumber::Four,
            String::from("five") => WrittenOutNumber::Five,
            String::from("six") => WrittenOutNumber::Six,
            String::from("seven") => WrittenOutNumber::Seven,
            String::from("eight") => WrittenOutNumber::Eight,
            String::from("nine") => WrittenOutNumber::Nine,
        }
    }

    fn to_int(&self) {

    }
}
impl Calibration {
    fn parse(input: Lines<BufReader<File>>) -> Option<Calibration> {
        if let lines = input.flatten() {
            // let raw_lines: Vec<String> = lines.collect::<Vec<String>>();
            let mut raw_lines: Vec<String> = Vec::new();
            let mut parsed_lines: Vec<Vec<u32>> = Vec::new();
            let mut calibration_values: Vec<u32> = Vec::new();
            for line in lines {
                raw_lines.push(line);
                parsed_lines.push(line);
            }
            Some(Calibration{
                raw_lines,
                parsed_lines,
                calibration_values,
            })
        } else {
            None
        }
    }
}
// struct CalibrationDocument {
//     reader: BufReader<File>,
//     calibration_value: CalibrationValue,
// }

fn read_lines_again<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn part_two(){
    let cal_test = Calibration::parse(read_lines_again("input_small_pt2.txt").unwrap());
    println!("cal test {:?}", cal_test);
}

// fn main() {
//     let mut parsed_document: Vec<(String, String)> = Vec::new();
//     if let Ok(lines) = read_lines("input.txt") {
//         for line in lines.flatten() {
//             let (first_half, second_half) = line.split_at(line.len().div_ceil(2));
//             parsed_document.push((first_half.to_string(), second_half.chars().rev().collect::<String>()))
//         }
//     }
//     let mut total = 0;
//     for item in parsed_document {
//
//         let mut dig1: Option<char> = None;
//         let mut dig2: Option<char> = None;
//
//         for (index, a_char) in item.0.chars().enumerate() {
//
//             let other_char = item.1.chars().nth(index).unwrap_or('-');
//
//             if dig1.is_some() && dig2.is_some() {
//                 println!("Found two digits early: {}{}", dig1.unwrap(), dig2.unwrap());
//                 break;
//             }
//             if a_char.to_digit(10).is_some() && dig1.is_none(){
//                 dig1 = Some(a_char);
//             }
//             if other_char.to_digit(10).is_some() && dig2.is_none() {
//                 dig2 = Some(other_char);
//             }
//
//         }
//         if dig1.is_some() && dig2.is_some() {
//             let concatenated = format!("{}{}", dig1.unwrap(), dig2.unwrap());
//             match concatenated.parse::<i32>() {
//                 Ok(parsed_number) => total += parsed_number,
//                 Err(_) => eprintln!("Could not parse digit pair into number"),
//             }
//         }
//         if dig1.is_some() && dig2.is_none() {
//             let concatenated = format!("{}{}", dig1.unwrap(), dig1.unwrap());
//             match concatenated.parse::<i32>() {
//                 Ok(parsed_number) => total += parsed_number,
//                 Err(_) => eprintln!("Could not parse first digit as itself twice"),
//             }
//         }
//     }
//
//     println!("Final code is {total}");
// }
