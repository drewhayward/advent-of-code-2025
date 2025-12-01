use crate::solution::Solution;

pub struct Dial;

#[derive(Debug)]
enum Move {
    R(i32),
    L(i32),
}

impl Move {
    fn from_string(input: &str) -> Self {
        let dir = &input[..1];
        let clicks: i32 = input[1..].parse().expect("move should end in a valid u32");
        match dir {
            "R" => Move::R(clicks),
            "L" => Move::L(clicks),
            _ => panic!("only L and R are valid enums"),
        }
    }
}

impl Solution for Dial {
    fn part1(puzzle_input: String) -> String {
        let moves = puzzle_input.split_whitespace().map(Move::from_string);

        let mut num_zeros = 0;
        let mut position = 50;
        for move_ in moves {
            position = match move_ {
                Move::R(clicks) => (position + clicks).rem_euclid(100),
                Move::L(clicks) => (position - clicks).rem_euclid(100),
            };

            if position == 0 {
                num_zeros += 1;
            }
        }

        format!("{num_zeros}")
    }

    fn part2(puzzle_input: String) -> String {
        let moves = puzzle_input.split_whitespace().map(Move::from_string);

        let mut num_zeros = 0;
        let mut position = 50;
        for move_ in moves {
            dbg!(&move_);
            let abs_position = match move_ {
                Move::R(clicks) => position + clicks,
                Move::L(clicks) => position - clicks,
            };

            let new_position = dbg!(abs_position.rem_euclid(100));

            // for the R case the number of wraps, is the number of zeros touched, since
            // each wrap has to touch a zero
            num_zeros += dbg!(abs_position.div_euclid(100).abs());

            // When moving left and stopping on zero, we have one more zero than wrap
            if let Move::L(_) = move_ && new_position == 0 {
                dbg!("ending at 0, adding 1");
                num_zeros += 1;
            }

            // When moving left and starting on zero, don't count the first wrap
            if let Move::L(_) = move_ && position == 0 {
                dbg!("ending at 0, adding 1");
                num_zeros -= 1;
            }

            position = new_position;
        }

        format!("{num_zeros}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_owned();

        assert_eq!(Dial::part1(input), "3");
    }

    #[test]
    fn test_part_2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        .to_owned();

        assert_eq!(Dial::part2(input), "6");
    }

    #[test]
    fn test_overflow() {
        assert_eq!(Dial::part2("R700".to_owned()), "7");
        assert_eq!(Dial::part2("L700".to_owned()), "7");
    }

    #[test]
    fn test_edge_r() {
        assert_eq!(Dial::part2("R50".to_owned()), "1");
        assert_eq!(Dial::part2("R150".to_owned()), "2");
    }

    #[test]
    fn test_edge_l() {
        assert_eq!(Dial::part2("L50".to_owned()), "1");
        assert_eq!(Dial::part2("L150".to_owned()), "2");
    }
}
