use std::io;
use std::path::Path;
use std::{env, fs};

use advent_of_code_2025::day01::Dial;
use advent_of_code_2025::day02::GiftShop;
use advent_of_code_2025::day03::Lobby;
use advent_of_code_2025::day04::PrintDpmt;
use advent_of_code_2025::day05::Cafe;
use advent_of_code_2025::day06::Trash;
use advent_of_code_2025::day07::Laboratories;
use advent_of_code_2025::solution::Solution;

fn get_input(day: u64) -> io::Result<String> {
    let s = format!("inputs/day{day}/input.txt");
    fs::read_to_string(Path::new(&s))
}

fn run_solutions<T: Solution>(input: String) {
    // println!("Tests");
    // let test_answer1 = T::part1(test_input.clone());
    // println!("{test_answer1}");

    //let test_answer2 = T::part2(test_input);
    //println!("{test_answer2}");

    println!("Solutions");
    let answer1 = T::part1(input.clone());
    println!("{answer1}");

    let answer2 = T::part2(input);
    println!("{answer2}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u64 = args[1].parse().unwrap();

    // let test_input = get_test_input(day).expect("File is read correctly");
    let input = get_input(day).expect("File is read correctly");

    match day {
        1 => run_solutions::<Dial>(input),
        2 => run_solutions::<GiftShop>(input),
        3 => run_solutions::<Lobby>(input),
        4 => run_solutions::<PrintDpmt>(input),
        5 => run_solutions::<Cafe>(input),
        6 => run_solutions::<Trash>(input),
        7 => run_solutions::<Laboratories>(input),
        _ => println!("No day solution for day {day}"),
    }
}
