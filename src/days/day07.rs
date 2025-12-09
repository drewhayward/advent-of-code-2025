use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

type Point = (i32, i32);

fn add(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1)
}

pub struct Laboratories;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

struct Grid {
    cells: HashMap<Point, Cell>,
    start: Point,
    width: i32,
    height: i32,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut cells = HashMap::new();
        let mut start = (0, 0);
        let height = input.lines().count() as i32;
        let width = input.lines().next().map(|l| l.len()).unwrap_or(0) as i32;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let cell = match c {
                    '.' => Cell::Empty,
                    'S' => {
                        start = (x as i32, y as i32);
                        Cell::Start
                    }
                    '^' => Cell::Splitter,
                    _ => panic!("Unknown cell type: {}", c),
                };
                cells.insert((x as i32, y as i32), cell);
            }
        }

        Self {
            cells,
            start,
            width,
            height,
        }
    }

    fn print_visited(&self, visited: &HashSet<Point>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                let c = match self.cells.get(&pos) {
                    Some(Cell::Empty) if visited.contains(&pos) => '|',
                    Some(Cell::Empty) => '.',
                    Some(Cell::Start) => 'S',
                    Some(Cell::Splitter) => '^',
                    None => ' ',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

impl Solution for Laboratories {
    fn part1(puzzle_input: String) -> String {
        let grid = Grid::parse(&puzzle_input);
        let mut frontier = vec![grid.start];
        let mut visited = HashSet::new();
        while let Some(position) = frontier.pop() {
            if visited.contains(&position) {
                continue;
            }
            visited.insert(position);

            let below = add(position, (0, 1));

            match grid.cells.get(&position) {
                Some(Cell::Empty) | Some(Cell::Start) => frontier.push(below),
                Some(Cell::Splitter) => {
                    frontier.push(add(position, (-1, 0)));
                    frontier.push(add(position, (1, 0)));
                }
                None => {}
            }
        }

        visited
            .iter()
            .filter(|p| matches!(grid.cells.get(p), Some(Cell::Splitter)))
            .count()
            .to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let grid = Grid::parse(&puzzle_input);

        // init starting state

        loop { // break when we get to the final tier

            // given this current known level, calculate the next level
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        assert_eq!(Laboratories::part1(TEST_INPUT.to_owned()), "21");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Laboratories::part2(TEST_INPUT.to_owned()), "40");
    }
}
