use std::fs;

fn main() {
    let file_string  = fs::read_to_string("input_small.txt")
        .expect("File not found");

    let lines: Vec<&str> = file_string.split("\n").collect();

    let mut total = 0;
    for line in lines {
        let _characters: Vec<&str> = line.split("").collect();
        decode(_characters);
    }
}

fn decode(line: Vec<&str>) {
    println!("{:?} /> {:?}", line, &line[]);
}
