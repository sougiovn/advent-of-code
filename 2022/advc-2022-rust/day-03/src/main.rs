use utils;

const lower_ratio: u8 = 96;
const upper_ratio: u8 = 38;

fn main() {
    let input = utils::get_input();

    part_1(&input);
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
    
}

#[derive(Debug)]
struct Rucksack<'a>(&'a str, &'a str);

fn parse_rucksack(input: &str) -> Vec<Rucksack> {
    input.lines()
        .map(|line| {
            let pivot = line.len()/2;
            Rucksack(
                &line[..pivot], 
                &line[pivot..]
            )
        })
        .collect()
}

fn find_shared_item(rucksack: &Rucksack) -> char {
    let mut shared_item: char = ' ';

    for letter in rucksack.0.chars() {
        if rucksack.1.contains(letter) {
            shared_item = letter;
            break;
        }
    }

    shared_item
}

fn map_priority(letter: char) -> i32 {
    let priority = letter as u8;

    if priority > lower_ratio {
        return (priority - lower_ratio).into();
    }

    (priority - upper_ratio).into()
}