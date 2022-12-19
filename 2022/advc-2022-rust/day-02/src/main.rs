use std::cmp::Ordering;

use utils::get_input;

const WIN: i32 = 6;
const DRAW: i32 = 3;

fn main() {
    let input = get_input();

    let mut total_score = 0;

    for line in input.lines() {
        let split = line.split_once(" ").unwrap();

        let opponent_play = Janken::from(split.0);

        let my_play: Janken = Janken::from(split.1);

        if my_play > opponent_play {
            total_score += WIN;
        } else if my_play == opponent_play {
            total_score += DRAW;
        }

        total_score += i32::from(my_play);
    }

    println!("My total score would be: {}", total_score);
}

#[derive(Eq, PartialEq, Debug)]
enum Janken {
    Rock,
    Paper,
    Scissors,
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
