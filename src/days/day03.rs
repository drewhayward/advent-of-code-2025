use std::{cmp::max, collections::HashMap};

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

/// concat adjacent digits
fn combine(digits: &[i64]) -> i64 {
    let mut acc = 0;
    for d in digits {
        acc = acc * 10 + d;
    }

    acc
}

fn max_substring(digits: &[i64], length: usize) -> i64 {
    let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
    // Build mem table for ascending sizes
    for l in 1..=length {
        for pos in (0..digits.len() - l + 1).rev() {
            // base case, there are only l digits
            if pos + l == digits.len() {
                let val = combine(&digits[pos..pos + l]);
                memo.insert((pos, l), val);
                continue;
            }

            // what is the max substring starting at pos of length l? either:
            
            // 1) we don't use the current digit and lookup the best result
            // from the suffix
            let max_suffix: i64 = *memo
                .get(&(pos + 1, l))
                .expect("should have just visited this cell");

            // 2) We use this digit and (possibly) the best suffix of l-1
            let use_digit: i64 = match l {
                1 => digits[pos],
                _ => {
                    let max_short_suffix = *memo
                        .get(&(pos + 1, l - 1))
                        .expect("should have visited this shorter cell");
                    let num_digits = (max_short_suffix as f64).log10().floor() as u32 + 1;
                    digits[pos] * 10_i64.pow(num_digits) + max_short_suffix
                }
            };

            memo.insert((pos, l), max(max_suffix, use_digit));
        }
    }

    *memo
        .get(&(0, length))
        .expect("should have populated on this cell")
}

pub struct Lobby;

impl Solution for Lobby {
    fn part1(puzzle_input: String) -> String {
        parse_banks(puzzle_input)
            .iter()
            .map(|b| max_substring(b, 2))
            .sum::<i64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        parse_banks(puzzle_input)
            .iter()
            .map(|b| max_substring(b, 12))
            .sum::<i64>()
            .to_string()
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
        assert_eq!(Lobby::part2(TEST_INPUT.to_owned()), "3121910778619");
    }

    #[test]
    fn test_simple() {
        assert_eq!(max_substring(&vec![9, 1, 2], 2), 92);
    }
}
