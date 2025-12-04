use std::fmt::format;

use crate::solution::Solution;

fn parse_banks(puzzle_input: String) -> Vec<Vec<i64>> {
    puzzle_input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i64>().expect("should be a digit"))
                .collect()
        })
        .collect()
}

pub struct Lobby;

impl Solution for Lobby {
    fn part1(puzzle_input: String) -> String {
        let banks = parse_banks(puzzle_input);

        let mut total: i64 = 0;
        for bank in banks {
            let mut max = 0;

            for (i, d1) in bank.iter().enumerate() {
                if i == bank.len() {
                    break
                }

                for d2 in &bank[i+1..] {
                    max = max.max(d1*10+d2);
                }
            }

            total += max;
        }

        total.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(Lobby::part1(TEST_INPUT.to_owned()), "357");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Lobby::part2(TEST_INPUT.to_owned()), "");
    }
}
