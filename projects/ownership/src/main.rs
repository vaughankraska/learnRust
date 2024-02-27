fn main() {

    {
        let str = "hello";
        let string = String::from("hej");
    } // scope of ends and is no longer valid

    let mut s = String::from("hellow");
    s.push_str(", world");

    let mut x = 5;
    let y = x;
    println!("x: {x}, y: {y}, x==y? {}", x == y);
    println!("Change x to ++");
    x += 1;
    println!("x: {x}, y: {y}, x==y? {}", x == y);

    // With strings !! no compile
    // let mut s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1: {s1}, s2: {s2}, s1==s2? {}", s1 == s2);
    // println!("Change s1");
    // s1.push_str(" change");
    // println!("s1: {s1}, s2: {s2}, s1==s2? {}", s1 == s2);

    // this also does not run. This concept is called MOVING/MOVED. It just changes the pointer
    // let s1 = String::from("hellow");
    // let s2 = s1;
    // println!("{}, world", s1);

    let s1 = String::from("Im gonna get cloned");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

}
