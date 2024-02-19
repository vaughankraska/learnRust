fn main() {
    let number = 5;

    if number < 5 {
        println!("condition true");
    } else {
        println!("condition false");
    };

    if number == 3 {
        println!("number was three");
    }

    if number % 4 == 0 {
        println!("number divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    let condition = false;
    let number = if condition { 5 } else { 1 }; // shadowing?
    println!("number: {number}");

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The loop result is {result}");
    println!();

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");
}
