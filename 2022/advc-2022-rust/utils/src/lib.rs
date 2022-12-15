use std::fs;

pub fn get_input() -> String {
  fs::read_to_string("./input")
    .expect("input not found")
}