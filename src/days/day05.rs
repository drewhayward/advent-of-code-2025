use crate::solution::Solution;

pub struct Cafe;

#[derive(Debug, PartialEq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from_string(input: &str) -> Self {
        let mut nums = input.split("-").map(|n| n.parse().unwrap());

        Range {
            start: nums.next().unwrap(),
            end: nums.next().unwrap(),
        }
    }
}

fn parse_input(puzzle_input: String) -> (Vec<Range>, Vec<i64>) {
    let (ranges, ingredients) = puzzle_input.split_once("\n\n").unwrap();

    let ranges = ranges.split_whitespace().map(Range::from_string).collect();
    let ingredients = ingredients
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    (ranges, ingredients)
}

impl Solution for Cafe {
    fn part1(puzzle_input: String) -> String {
        let (ranges, ingredients) = parse_input(puzzle_input);

        let mut total = 0;
        for ingr in ingredients {
            for range in &ranges {
                if ingr >= range.start && ingr <= range.end {
                    total += 1;
                    break;
                }
            }
        }

        total.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let (mut ranges, _) = parse_input(puzzle_input);

        // Keep ranges in ascending order by start
        ranges.sort_by(|x, y| x.start.cmp(&y.start));

        let mut merged: Vec<Range> = Vec::new();
        let mut current = None;
        for next in ranges {
            match current {
                None => current = Some(next),
                Some(ref mut c) if c.end >= next.start => c.end = c.end.max(next.end),
                Some(c) => {
                    merged.push(c);
                    current = Some(next);
                }
            };
        }

        if let Some(current)  = current {
            merged.push(current);
        }


        merged
            .iter()
            .map(|r| r.end - r.start + 1)
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        assert_eq!(Cafe::part1(TEST_INPUT.to_owned()), "3");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Cafe::part2(TEST_INPUT.to_owned()), "14");
    }
}
