fn main() {
    let s = String::from("hellow");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{}", x); // this is able to run because numbers are Copy -ed.

    let s1 = gives_ownership();
    let s2 = String::from("hellow");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {s1}, s2 = was given to s3 by takes_and_gives_back, s3 = {s3}");

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Im all yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("Im just taking this -{}- for a lil bit", a_string);

    a_string
}