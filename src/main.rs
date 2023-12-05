mod day01p1;
mod day01p2;
mod day02p1;
mod day02p2;
mod day03p1;
mod day03p2;
mod day04p1;
mod day04p2;
mod day05;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let day = args.next().unwrap().parse().unwrap();
    let puzzle = args.next().unwrap().parse().unwrap();
    let input = fs::read_to_string(args.next().unwrap()).unwrap();
    let answer = match (day, puzzle) {
        (1, 1) => day01p1::puzzle_1(input),
        (1, 2) => day01p2::puzzle_2(input),
        (2, 1) => day02p1::puzzle_1(input),
        (2, 2) => day02p2::puzzle_2(input),
        (3, 1) => day03p1::puzzle_1(input),
        (3, 2) => day03p2::puzzle_2(input),
        (4, 1) => day04p1::puzzle_1(input),
        (4, 2) => day04p2::puzzle_2(input),
        (5, 1) => day05::puzzle_1(input),
        (5, 2) => day05::puzzle_2(input),

        _ => panic!("No puzzle {} for day {}", puzzle, day),
    };
    println!("{}", answer);
}