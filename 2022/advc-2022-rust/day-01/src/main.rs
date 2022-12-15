use utils;

fn main() {
    let input = utils::get_input();
    
    let mut current_calories = 0;
    let mut most_calories = 0;
    
    for line in input.lines() {
      if line.is_empty() {
        if current_calories > most_calories {
          most_calories = current_calories;
        }
        current_calories = 0;
        continue;
      }

      current_calories += line.parse::<i32>().unwrap();
    }

    println!("Elf with most calories has: {}", most_calories);
}
