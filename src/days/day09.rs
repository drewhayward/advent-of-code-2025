use std::cmp::{max, min};
use std::ops::{Add, Sub};

use crate::solution::Solution;

pub struct Theater;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn lines_intersect(line1: Line, line2: Line) -> bool {
    true
}

impl Solution for Theater {
    fn part1(puzzle_input: String) -> String {
        let coords = parse_input(&puzzle_input);

        let mut area = 0;
        for coord1 in &coords {
            for coord2 in &coords {
                let width = max(coord1.x, coord2.x) - min(coord1.x, coord2.x) + 1;
                let height = max(coord1.y, coord2.y) - min(coord1.y, coord2.y) + 1;

                area = area.max(width * height)
            }
        }

        area.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let coords = parse_input(&puzzle_input);
        let lines: Vec<_> = coords.windows(2).map(|w| Line::new(w[0], w[1])).collect();
        // lines.push(Line::new(*coords.last().unwrap(), *coords.first().unwrap()));

        for a in &lines {
            for b in &lines {
                if a == b {
                    continue;
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1() {
        assert_eq!(Theater::part1(TEST_INPUT.to_owned()), "50");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Theater::part2(TEST_INPUT.to_owned()), "24");
    }
}
