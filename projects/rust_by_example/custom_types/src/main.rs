use crate::enums::{inspect, play_with_linked_list_enum, WebEvent};
use crate::structures::{destructuring, exercise};

mod structures;
mod enums;

fn main() {
    exercise();
    destructuring();
    inspect(WebEvent::Click {x: 0, y: 1});
    inspect(WebEvent::KeyPress('t'));
    play_with_linked_list_enum();
}
