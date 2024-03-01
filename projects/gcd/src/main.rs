use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg).expect("Error parsing input"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = greatest_common_denominator(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn greatest_common_denominator(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m = m % n;
    }

    n
}