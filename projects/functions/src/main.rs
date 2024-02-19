fn main() {
    println!("Hello world");
    another_function(3);
    print_labeled_measurement(300, 'f');

    let y = {
        let x = 4;
        x + 1
    };
    println!("the value of y: {y}");
    let five = returns_five();
    println!("the returns five function returns: {} and {five} as a variable.", returns_five());
    println!("function returns five == 5? {}", five == 5);
    println!("function returns five == variable five? {}", five == returns_five());
    println!();
    println!("The value of x is: {}", plus_one(five));
}

fn another_function(x: i32) {
    println!("Another function x: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn returns_five() ->i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}