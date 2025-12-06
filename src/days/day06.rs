use std::{
    iter::zip,
    ops::{Add, Mul},
};

use crate::solution::Solution;

pub struct Trash;

fn parse_input(puzzle_input: String) -> (Vec<Vec<i64>>, Vec<char>) {
    let mut lines: Vec<&str> = puzzle_input.lines().collect();
    let ops_line = lines.pop().unwrap();

    let numbers: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<char> = ops_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    (numbers, ops)
}

fn transpose<T: Copy>(nums: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for j in 0..nums[0].len() {
        let mut column = Vec::new();
        for i in 0..nums.len() {
            column.push(nums[i][j]);
        }
        result.push(column)
    }
    result
}

impl Solution for Trash {
    fn part1(puzzle_input: String) -> String {
        let (numbers, ops) = parse_input(puzzle_input);

        let numbers = transpose(numbers);
        zip(numbers, ops)
            .map(|(nums, op)| {
                let (acc, op): (i64, fn(i64, i64) -> i64) = match op {
                    '*' => (1, Mul::mul),
                    '+' => (0, Add::add),
                    _ => panic!("nope"),
                };

                nums.iter().fold(acc, |a, b| op(a, *b))
            })
            .sum::<i64>()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let height = puzzle_input.lines().count();
        let width = puzzle_input.lines().map(|l| l.len()).max().unwrap();

        let mut grid: Vec<Vec<char>> = vec![vec![' '; height]; width];

        let mut ops: Vec<&str> = Vec::new();
        for (y, line) in puzzle_input.lines().enumerate() {
            if y == height - 1 {
                ops = line.split_whitespace().filter(|s| !s.is_empty()).collect();
                break;
            }

            for (x, c) in line.chars().enumerate() {
                grid[width - x - 1][y] = c;
            }
        }

        grid = grid
            .iter()
            .map(|l| l.into_iter().filter(|s| **s != ' ').map(|c| *c).collect())
            .collect();

        // dbg!(grid);
        let problems: Vec<Vec<i64>> = grid
            .split(|l| l.is_empty())
            .map(|problems| {
                problems.iter().map(|digits| {
                    digits
                        .iter()
                        .fold(0_i64, |acc, d| acc * 10 + d.to_digit(10).unwrap() as i64)
                }).collect()
            })
            .collect();

        zip(dbg!(problems), ops.iter().rev())
            .map(|(nums, op)| {
                let (acc, op): (i64, fn(i64, i64) -> i64) = match *op {
                    "*" => (1, Mul::mul),
                    "+" => (0, Add::add),
                    _ => panic!("nope"),
                };

                nums.iter().fold(acc, |a, b| op(a, *b))
            })
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";

    #[test]
    fn test_part_1() {
        assert_eq!(Trash::part1(TEST_INPUT.to_owned()), "4277556");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Trash::part2(TEST_INPUT.to_owned()), "3263827");
    }
}
