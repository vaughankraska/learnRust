use crate::enums::List::*;
#[allow(dead_code)]

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                "Nil".to_string()
            },
        }
    }
}

pub fn play_with_linked_list_enum() {
    println!(r#"
    #####################
    enums 3.2.3 linked list with enums
    #####################
    "#);
    let mut linked_list = List::new();
    linked_list = linked_list.prepend(1);
    println!("{}", linked_list.stringify());
    linked_list = linked_list.prepend(2);
    println!("{}", linked_list.stringify());

}

pub enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
pub fn inspect(event: WebEvent) {
    println!(r#"
    ##############
    enums 3.2
    ##############
    "#);
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}


