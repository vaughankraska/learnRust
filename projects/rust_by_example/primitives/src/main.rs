mod arrays;
mod tuples;

fn main() {
    let t = (123, false);
    let reversed_tuple = tuples::reverse(t);
    println!("before: {:?}, after: {:?}", t, reversed_tuple);

    tuples::exercise();
    arrays::example();
}
