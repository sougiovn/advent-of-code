use std::{cmp::Ordering, collections::binary_heap::Iter};

use utils;

const WIN: i32 = 6;
const DRAW: i32 = 3;

fn main() {
    let input = utils::get_input();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut total_score = 0;

    for round in parse_rounds(&input) {
        if round.my_play > round.opponent_play {
            total_score += WIN;
        } else if round.my_play == round.opponent_play {
            total_score += DRAW;
        }

        total_score += i32::from(round.my_play);
    }

    println!("My total score would be: {}", total_score);
}

fn part_2(input: &str) {
    let mut total_score = 0;

    for round in parse_rounds(&input) {
        match round.expected_result {
            RoundResult::Win => {
                total_score += WIN;
                total_score += i32::from(round.opponent_play.to_win());
            },
            RoundResult::Draw => {
                total_score += DRAW;
                total_score += i32::from(round.opponent_play.to_draw());
            },
            RoundResult::Lose => {
                total_score += i32::from(round.opponent_play.to_lose());
            }
        }
    }

    println!("My total score following the strategy would be: {}", total_score);
}

fn parse_rounds(input: &str) -> Vec<Round> {
    input.lines()
        .map(|line| {
            let plays = line.split_once(" ").unwrap();
            Round {
                opponent_play: Janken::from(plays.0), 
                my_play: Janken::from(plays.1),
                expected_result: RoundResult::from(plays.1)
            }
        })
        .collect()
}

struct Round {
    opponent_play: Janken,
    my_play: Janken,
    expected_result: RoundResult
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Janken {
    Rock,
    Paper,
    Scissors,
}

impl Janken {
    fn to_win(&self) -> Self {
        match self {
            Janken::Rock => Janken::Paper,
            Janken::Paper => Janken::Scissors,
            Janken::Scissors => Janken::Rock
        }
    }
    fn to_draw(&self) -> Self {
        self.clone()
    }
    fn to_lose(&self) -> Self {
        match self {
            Janken::Rock => Janken::Scissors,
            Janken::Paper => Janken::Rock,
            Janken::Scissors => Janken::Paper
        }
    }
}

/// Parsing &str values to Janken enum
impl From<&str> for Janken {
    fn from(s: &str) -> Janken {
        match s {
            // column 1
            "A" => Janken::Rock,
            "B" => Janken::Paper,
            "C" => Janken::Scissors,
            // column 2
            "X" => Janken::Rock,
            "Y" => Janken::Paper,
            "Z" => Janken::Scissors,
            _ => panic!("invalid play"),
        }
    }
}

/// Parsing Janken enum to each of its points
impl From<Janken> for i32 {
    fn from(janken: Janken) -> Self {
        match janken {
            Janken::Rock => 1,
            Janken::Paper => 2,
            Janken::Scissors => 3,
        }
    }
}

/// Ordering its win/draw/lose
impl PartialOrd for Janken {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        })
    }
}

enum RoundResult {
    Win,
    Draw,
    Lose
}

impl From<&str> for RoundResult {
    fn from(s: &str) -> RoundResult {
        match s {
            "X" => RoundResult::Lose,
            "Y" => RoundResult::Draw,
            "Z" => RoundResult::Win,
            _ => panic!("invalid play"),
        }
    }
}

mod test {
    use crate::Janken;

    // Rock
    #[test]
    fn rock_win_scissors() {
        assert!(Janken::Rock > Janken::Scissors);
    }
    #[test]
    fn rock_draw_rock() {
        assert!(Janken::Rock == Janken::Rock);
    }

    #[test]
    fn rock_lose_paper() {
        assert!(Janken::Rock < Janken::Paper);
    }

    // Paper
    #[test]
    fn paper_win_rock() {
        assert!(Janken::Paper > Janken::Rock);
    }
    #[test]
    fn paper_draw_paper() {
        assert!(Janken::Paper == Janken::Paper);
    }

    #[test]
    fn paper_lose_scissors() {
        assert!(Janken::Paper < Janken::Scissors);
    }

    // Scissors
    #[test]
    fn scissors_win_paper() {
        assert!(Janken::Scissors > Janken::Paper);
    }
    #[test]
    fn scissors_draw_scissors() {
        assert!(Janken::Scissors == Janken::Scissors);
    }

    #[test]
    fn scissors_lose_rock() {
        assert!(Janken::Scissors < Janken::Rock);
    }
}
