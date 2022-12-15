use std::cmp::Ordering;

use utils::get_input;

const WIN: i32 = 6;
const DRAW: i32 = 3;

fn main() {
    let input = get_input();

    let mut total_score = 0;

    for line in input.lines() {
      let split = line.split_once(" ").unwrap();
      
      println!("Round: {:?}", &split);

      let opponent_play = Janken::from(split.0);  

      let my_play: Janken = Janken::from(split.1);

      println!("{:?} x {:?}", &opponent_play, &my_play);

      if my_play > opponent_play{
        total_score += WIN;
        println!("WIN: {}", &total_score);
      } else if my_play == opponent_play {
        total_score += DRAW;
        println!("DRAW: {}", &total_score);
      }

      total_score += i32::from(my_play);
      println!("Round result: {}", &total_score);
      println!("-----");
    }

  println!("My total score would be: {}", total_score);
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum Janken {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Janken {
  fn from(s: &str) -> Janken {
    match s {
        "A" => Janken::Rock,
        "B" => Janken::Paper,
        "C" => Janken::Scissor,
        "X" => Janken::Rock,
        "Y" => Janken::Paper,
        "Z" => Janken::Scissor,
        _ => panic!("invalid play")
    }
  }
}

impl From<Janken> for i32 {
  fn from(janken: Janken) -> Self {
      match janken {
        Janken::Rock => 1,
        Janken::Paper => 2,
        Janken::Scissor => 3,
      }
  }
}

impl Ord for Janken {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissor => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissor => Ordering::Less,
            },
            Self::Scissor => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissor => Ordering::Equal,
            },
        }
    }
}
