use utils;

fn main() {
    let input = utils::get_input();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut most_calories = 0;

    for elf in sum_elves_snacks(input) {
        if elf > most_calories {
            most_calories = elf;
        }
    }

    println!("Elf with most calories has: {}", most_calories);
}

fn part_2(input: &str) {
    let mut elves_calories: Vec<i32> = sum_elves_snacks(input);
    elves_calories.sort();
    elves_calories.reverse();

    let top_3: i32 = elves_calories[..3].into_iter().sum();

    println!("Top 3 elves are carrying {} calories", top_3);
}

fn sum_elves_snacks(input: &str) -> Vec<i32> {
    let mut elves_calories = Vec::new();

    let mut current_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves_calories.push(current_calories);
            current_calories = 0;
            continue;
        }

        current_calories += line.parse::<i32>().unwrap();
    }

    if current_calories > 0 {
        elves_calories.push(current_calories);
    }

    elves_calories
}
