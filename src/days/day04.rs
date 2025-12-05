use std::collections::HashMap;

use crate::solution::Solution;

type Shelves = HashMap<Point, Cell>;
type Point = (i64, i64);

const NEIGHBORS: [Point; 8] = [
    (0, -1),  // U
    (0, 1),   // D
    (-1, 0),  // L
    (1, 0),   // R
    (-1, -1), // UL
    (1, -1),  // UR
    (-1, 1),  // DL
    (1, 1),   // DR
];

enum Cell {
    Empty,
    Paper,
}

fn parse_input(puzzle_input: String) -> Shelves {
    puzzle_input
        .split_whitespace()
        .enumerate()
        .flat_map(|(y, row)| {
            let cells: Vec<_> = row
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x as i64, y as i64),
                        match c {
                            '.' => Cell::Empty,
                            '@' => Cell::Paper,
                            _ => panic!("bad input"),
                        },
                    )
                })
                .collect();
            cells
        })
        .collect()
}

fn can_remove(shelves: &Shelves, point: &Point) -> bool {
    let (x, y) = point;
    NEIGHBORS
        .iter()
        .filter(|(dx, dy)| matches!(shelves.get(&(x + dx, y + dy)), Some(Cell::Paper)))
        .count() < 4
}

pub struct PrintDpmt;

impl Solution for PrintDpmt {
    fn part1(puzzle_input: String) -> String {
        let shelves = parse_input(puzzle_input);

        let mut total = 0;
        // For every position in the grid
        for (point, cell) in &shelves {
            if let Cell::Empty = cell {
                continue;
            }
            if can_remove(&shelves, point) {
                total += 1
            }
        }
        // check all the negihbors and count
        total.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let mut shelves = parse_input(puzzle_input);

        let mut removed = 0;

        loop {
            let positions: Vec<_> = shelves
                .iter()
                .filter_map(|(point, cell)| match cell {
                    Cell::Paper => Some(*point),
                    Cell::Empty => None,
                })
                .collect();

            let mut did_remove = false;
            for pos in positions {
                if can_remove(&shelves, &pos) {
                    shelves.insert(pos, Cell::Empty);
                    removed += 1;
                    did_remove = true;
                }
            }

            if !did_remove {
                break;
            }
        }

        removed.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        assert_eq!(PrintDpmt::part1(TEST_INPUT.to_owned()), "13");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(PrintDpmt::part2(TEST_INPUT.to_owned()), "43");
    }
}
