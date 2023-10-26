// A for Rock, B for Paper, and C for Scissors.
//(1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round
//(0 if you lost, 3 if the round was a draw, and 6 if you won)

use anyhow::{Ok, Result};
use std::{cmp::Ordering, fs, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Round(u32);

fn part_1(filename: &str) -> u32 {
    impl FromStr for Move {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self> {
            match s {
                "A" | "X" => Ok(Move::Rock),
                "B" | "Y" => Ok(Move::Paper),
                "C" | "Z" => Ok(Move::Scissors),
                _ => Err(anyhow::format_err!("Could not parse Move")),
            }
        }
    }

    impl PartialOrd for Move {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self == &Move::Scissors && other == &Move::Rock {
                Some(Ordering::Less)
            } else if other == &Move::Scissors && self == &Move::Rock {
                Some(Ordering::Greater)
            } else {
                Some((*self as u8).cmp(&(*other as u8)))
            }
        }
    }

    impl FromStr for Round {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self> {
            let moves: Vec<Move> = s.split(" ").map(|s| s.parse::<Move>().unwrap()).collect();

            let op = moves[0];
            let mine = moves[1];
            match mine.partial_cmp(&op) {
                Some(Ordering::Equal) => Ok(Round(3 + mine as u32)),
                Some(Ordering::Greater) => Ok(Round(6 + mine as u32)),
                Some(Ordering::Less) => Ok(Round(mine as u32)),
                None => panic!("moves should be comparable"),
            }
        }
    }
    aoc::read_one_per_line::<Round>(filename)
        .unwrap()
        .iter()
        .map(|round| round.0 as u32)
        .sum::<u32>()
}

fn secret_strategy(op: Move, mov: &str) -> Result<u32> {
    match mov {
        "X" => {
            let res = match op {
                Move::Rock => Move::Scissors as u32,
                Move::Paper => Move::Rock as u32,
                Move::Scissors => Move::Paper as u32,
            };
            Ok(0 + res)
        }
        "Y" => Ok(3 + op as u32),
        "Z" => {
            let res = match op {
                Move::Rock => Move::Paper as u32,
                Move::Paper => Move::Scissors as u32,
                Move::Scissors => Move::Rock as u32,
            };
            Ok(6 + res)
        }
        _ => panic!("Unexpected right move"),
    }
}

fn part_2(filename: &str) -> u32 {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            let moves: Vec<&str> = line.split(" ").collect();
            let op_move = moves[0].parse::<Move>().unwrap();
            secret_strategy(op_move, moves[1]).unwrap_or(0)
        })
        .sum::<u32>()
}

fn main() {
    // unimplemented!();
    let filename = "day2.input";
    println!("result: {:?}", part_1(filename));
    println!("{filename}");

    println!("result_2: {:?}", part_2(filename));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let filename = "src/day2.test";
        assert_eq!(part_1(filename), 15)
    }
}
