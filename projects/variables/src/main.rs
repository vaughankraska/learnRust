use std::io;

fn main() {
    let x = 5;

    let x = x + 1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let mut spaces = "    ";
    println!("Spaces length: {}", spaces.len());

    // working with arrays
    let a = [1, 2, 3, 4, 5];
    println!("enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read the line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    let array: [u8; 5] = [1, 2, 3, 4, 255]; // 255 is the max size of u8


}
