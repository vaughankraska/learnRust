extern crate core;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut parsed_document: Vec<(String, String)> = Vec::new();
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines.flatten() {
            let (first_half, second_half) = line.split_at(line.len().div_ceil(2));
            parsed_document.push((first_half.to_string(), second_half.chars().rev().collect::<String>()))
        }
    }
    let mut total = 0;
    for item in parsed_document {

        let mut dig1: Option<char> = None;
        let mut dig2: Option<char> = None;

        for (index, a_char) in item.0.chars().enumerate() {

            let other_char = item.1.chars().nth(index).unwrap_or('-');

            if dig1.is_some() && dig2.is_some() {
                println!("Found two digits early: {}{}", dig1.unwrap(), dig2.unwrap());
                break;
            }
            if a_char.to_digit(10).is_some() && dig1.is_none(){
                dig1 = Some(a_char);
            }
            if other_char.to_digit(10).is_some() && dig2.is_none() {
                dig2 = Some(other_char);
            }

        }
        if dig1.is_some() && dig2.is_some() {
            let concatenated = format!("{}{}", dig1.unwrap(), dig2.unwrap());
            match concatenated.parse::<i32>() {
                Ok(parsed_number) => total += parsed_number,
                Err(_) => eprintln!("Could not parse digit pair into number"),
            }
        }
        if dig1.is_some() && dig2.is_none() {
            let concatenated = format!("{}{}", dig1.unwrap(), dig1.unwrap());
            match concatenated.parse::<i32>() {
                Ok(parsed_number) => total += parsed_number,
                Err(_) => eprintln!("Could not parse first digit as itself twice"),
            }
        }
    }

    println!("Final code is {total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}