use std::fs;

pub fn get_input() -> String {
  fs::read_to_string("./input")
    .expect("input not found")
}

pub fn get_sample() -> String {
  fs::read_to_string("./sample")
    .expect("sample not found")
}