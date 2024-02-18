use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("guess the number");
    println!("the secret number is {secret_number}");
    loop {
        println!("input your guess");
        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("Guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("WIN!");
                break;
            },
        }
    }
}
