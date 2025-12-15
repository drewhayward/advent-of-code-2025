use std::collections::HashMap;

use crate::solution::Solution;

pub struct Reactor;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (name, children) = line.split_once(": ").unwrap();
            let children: Vec<&str> = children.split_whitespace().collect();
            (name, children)
        })
        .collect()
}

fn count_paths<'a>(
    edge_list: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, bool, bool), u64>,
    node: &'a str,
    seen_dac: bool,
    seen_fft: bool,
) -> u64 {
    let seen_dac = seen_dac || node == "dac";
    let seen_fft = seen_fft || node == "fft";

    if node == "out" {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    if let Some(&cached) = memo.get(&(node, seen_dac, seen_fft)) {
        return cached;
    }

    let mut total = 0;
    if let Some(children) = edge_list.get(node) {
        for child in children {
            total += count_paths(edge_list, memo, child, seen_dac, seen_fft);
        }
    }

    memo.insert((node, seen_dac, seen_fft), total);
    total
}

impl Solution for Reactor {
    fn part1(puzzle_input: String) -> String {
        let edge_list = parse_input(&puzzle_input);

        // DFS over the edge list
        let mut frontier = vec!["you"];
        let mut num_ways = 0;
        while let Some(device) = frontier.pop() {
            if device == "out" {
                num_ways += 1;
            }
            if let Some(children) = edge_list.get(device) {
                for child in children {
                    frontier.push(*child);
                }
            }
        }

        num_ways.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let edge_list = parse_input(&puzzle_input);
        let mut memo = HashMap::new();

        count_paths(&edge_list, &mut memo, "svr", false, false).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TEST_INPUT_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_1() {
        assert_eq!(Reactor::part1(TEST_INPUT.to_owned()), "5");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(Reactor::part2(TEST_INPUT_2.to_owned()), "2");
    }
}
