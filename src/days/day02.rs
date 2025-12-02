use crate::solution::Solution;

struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn parse_from_string(input: &str) -> Range {
        let mut nums = input.split("-");
        let start = nums
            .next()
            .expect("should have a number in the range")
            .parse()
            .expect("Number should be valid");

        let end = nums
            .next()
            .expect("should have a number in the range")
            .parse()
            .expect("Number should be valid");

        Self { start, end }
    }
}

fn is_mirrored(digit: i64) -> bool {
    let digit_str = format!("{digit}");
    let midpoint = digit_str.len() / 2;

    digit_str[0..midpoint] == digit_str[midpoint..]
}

fn is_repeated(digit: i64) -> bool {
    let digit_str = format!("{digit}");
    for cycles in 2..=digit_str.len() {
        // if the digit doesn't divide evenly by our target number of cycles
        if digit_str.len() % cycles != 0 {
            continue;
        }

        // Build a list of parts
        let period = digit_str.len() / cycles;
        let mut parts = Vec::new();
        let mut current = 0;
        while current + period <= digit_str.len() {
            parts.push(&digit_str[current..current + period]);
            current = current + period
        }

        // Try to assert pairwise equality
        let parts_match = parts.windows(2).all(|window| {
            let left = window[0];
            let right = window[1];

            left == right
        });

        if parts_match {
            return true;
        }
    }

    false
}

pub struct GiftShop;

impl Solution for GiftShop {
    fn part1(puzzle_input: String) -> String {
        let ranges: Vec<_> = puzzle_input
            .split(&['\n', ','][..])
            .filter(|s| !s.is_empty())
            .map(Range::parse_from_string)
            .collect();

        let mut sum_invalid = 0;
        for range in ranges {
            for id in range.start..range.end + 1 {
                if is_mirrored(id) {
                    sum_invalid += id;
                }
            }
        }

        format!("{sum_invalid}")
    }

    fn part2(puzzle_input: String) -> String {
        let ranges: Vec<_> = puzzle_input
            .split(&['\n', ','][..])
            .filter(|s| !s.is_empty())
            .map(Range::parse_from_string)
            .collect();

        let mut sum_invalid = 0;
        for range in ranges {
            for id in range.start..range.end + 1 {
                if is_repeated(id) {
                    sum_invalid += id;
                }
            }
        }

        format!("{sum_invalid}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        assert_eq!(GiftShop::part1(TEST_INPUT.to_owned()), "1227775554");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(GiftShop::part2(TEST_INPUT.to_owned()), "4174379265");
    }

    #[test]
    fn test_repeated_cycle_2() {
        assert_eq!(is_repeated(11), true);
    }
}
