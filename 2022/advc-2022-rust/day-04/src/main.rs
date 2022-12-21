use std::{cmp::Ordering, str::FromStr};

use utils;

fn main() {
    let input = utils::get_input();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let total_overlaps: i32 = input
        .lines()
        .map(|line| parse_pair(line))
        .filter(|pair| has_full_overlap(pair))
        .count()
        .try_into()
        .unwrap();

    println!(
        "Total amount of pairs with full overlap is: {}",
        total_overlaps
    );
}

fn part_2(input: &str) {
    let total_overlaps: i32 = input
        .lines()
        .map(|line| parse_pair(line))
        .filter(|pair| has_overlap(pair))
        .count()
        .try_into()
        .unwrap();

    println!("Total amount of pairs with overlap is: {}", total_overlaps);
}

#[derive(Eq, PartialEq)]
struct Area {
    begin: i32,
    end: i32,
}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_size = self.end - self.begin;
        let other_size = other.end - other.begin;

        Some(if self_size > other_size {
            Ordering::Greater
        } else if self_size == other_size {
            Ordering::Equal
        } else {
            Ordering::Less
        })
    }
}

struct Pair(Area, Area);

fn parse_pair(line: &str) -> Pair {
    let pair: Vec<&str> = line.split(",").collect();
    let first_pair: Vec<&str> = pair[0].split("-").collect();
    let second_pair: Vec<&str> = pair[1].split("-").collect();

    Pair(
        Area {
            begin: FromStr::from_str(first_pair[0]).unwrap(),
            end: FromStr::from_str(first_pair[1]).unwrap(),
        },
        Area {
            begin: FromStr::from_str(second_pair[0]).unwrap(),
            end: FromStr::from_str(second_pair[1]).unwrap(),
        },
    )
}

fn has_full_overlap(pair: &Pair) -> bool {
    let bigger;
    let smaller;

    if pair.0 > pair.1 {
        bigger = &pair.0;
        smaller = &pair.1;
    } else {
        bigger = &pair.1;
        smaller = &pair.0;
    };

    smaller.begin >= bigger.begin && smaller.end <= bigger.end
}

fn has_overlap(pair: &Pair) -> bool {
    let bigger;
    let smaller;

    if pair.0 > pair.1 {
        bigger = &pair.0;
        smaller = &pair.1;
    } else {
        bigger = &pair.1;
        smaller = &pair.0;
    };

    (smaller.begin >= bigger.begin && smaller.begin <= bigger.end)
        || (smaller.end >= bigger.begin && smaller.end <= bigger.end)
}
