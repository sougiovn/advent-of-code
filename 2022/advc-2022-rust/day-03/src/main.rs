use std::{slice::Iter, num::NonZeroUsize};

use iter_chunks::{Chunks, IterChunks};
use utils;

const lower_ratio: u8 = 96;
const upper_ratio: u8 = 38;

fn main() {
    let input = utils::get_input();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let total_sum: i32 = parse_rucksack(input)
        .iter()
        .map(|rucksack| find_shared_item(rucksack))
        .map(|shared_item| map_priority(shared_item))
        .sum();

    println!("The total sum of priorities is: {}", total_sum);
}

fn part_2(input: &str) {
    let mut total_sum: i32 = 0;
    
    let rucksacks = parse_rucksack(input);
    let mut grouped_rucksacks = rucksacks.iter().chunks(3);

    while let Some(group) = grouped_rucksacks.next() {
        let rucksacks: Vec<&Rucksack> = group.collect();
        
        // Last chunk comes empty
        if rucksacks.len() == 0 {
            break;
        }

        let badge = find_badge(rucksacks);
        total_sum += map_priority(badge);
    }

    println!("The total sum of priorities from badges is: {}", total_sum);
}

#[derive(Debug)]
struct Rucksack<'a> {
    all: &'a str,
    compartiment_1: &'a str,
    compartiment_2: &'a str
}

fn parse_rucksack(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let pivot = line.len() / 2;
            Rucksack {
                all: line,
                compartiment_1: &line[..pivot],
                compartiment_2: &line[pivot..]
            }
        })
        .collect()
}

fn find_shared_item(rucksack: &Rucksack) -> char {
    let mut shared_item: char = ' ';

    for letter in rucksack.compartiment_1.chars() {
        if rucksack.compartiment_2.contains(letter) {
            shared_item = letter;
            break;
        }
    }

    shared_item
}

fn find_badge(rucksacks: Vec<&Rucksack>) -> char {
    let first = rucksacks[0];
    let second = rucksacks[1];
    let third = rucksacks[2];

    let mut badge = ' ';
    for letter in first.all.chars() {
        if second.all.contains(letter) && third.all.contains(letter) {
            badge = letter;
            break;
        }
    }

    badge
}

fn map_priority(letter: char) -> i32 {
    let priority = letter as u8;

    if priority > lower_ratio {
        return (priority - lower_ratio).into();
    }

    (priority - upper_ratio).into()
}
