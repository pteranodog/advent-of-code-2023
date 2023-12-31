mod day01p1;
mod day01p2;
mod day02p1;
mod day02p2;
mod day03p1;
mod day03p2;
mod day04p1;
mod day04p2;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let day = args.next().unwrap().parse().unwrap();
    let puzzle = args.next().unwrap().parse().unwrap();
    let input = fs::read_to_string(args.next().unwrap()).unwrap();
    let answer: i128 = match (day, puzzle) {
        (1, 1) => day01p1::puzzle_1(input).into(),
        (1, 2) => day01p2::puzzle_2(input).into(),
        (2, 1) => day02p1::puzzle_1(input).into(),
        (2, 2) => day02p2::puzzle_2(input).into(),
        (3, 1) => day03p1::puzzle_1(input).into(),
        (3, 2) => day03p2::puzzle_2(input).into(),
        (4, 1) => day04p1::puzzle_1(input).into(),
        (4, 2) => day04p2::puzzle_2(input).into(),
        (5, 1) => day05::puzzle_1(input).try_into().unwrap(),
        (5, 2) => day05::puzzle_2(input).try_into().unwrap(),
        (6, 1) => day06::puzzle_1(input).into(),
        (6, 2) => day06::puzzle_2(input).into(),
        (7, 1) => day07::puzzle_1(input).into(),
        (7, 2) => day07::puzzle_2(input).into(),
        (8, 1) => day08::puzzle_1(input).into(),
        (8, 2) => day08::puzzle_2(input).into(),
        (9, 1) => day09::puzzle_1(input).into(),
        (9, 2) => day09::puzzle_2(input).into(),
        (10, 1) => day10::puzzle_1(input).into(),
        (10, 2) => day10::puzzle_2(input).into(),
        (11, 1) => day11::puzzle_1(input).into(),
        (11, 2) => day11::puzzle_2(input).into(),


        _ => panic!("No puzzle {} for day {}", puzzle, day),
    };
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day01p1() {
        let input = std::fs::read_to_string("input-test/day01p1.txt").unwrap();
        assert_eq!(super::day01p1::puzzle_1(input), 142);
    }

    #[test]
    fn day01p2() {
        let input = std::fs::read_to_string("input-test/day01p2.txt").unwrap();
        assert_eq!(super::day01p2::puzzle_2(input), 281);
    }

    #[test]
    fn day02p1() {
        let input = std::fs::read_to_string("input-test/day02.txt").unwrap();
        assert_eq!(super::day02p1::puzzle_1(input), 8);
    }

    #[test]
    fn day02p2() {
        let input = std::fs::read_to_string("input-test/day02.txt").unwrap();
        assert_eq!(super::day02p2::puzzle_2(input), 2286);
    }

    #[test]
    fn day03p1() {
        let input = std::fs::read_to_string("input-test/day03.txt").unwrap();
        assert_eq!(super::day03p1::puzzle_1(input), 4361);
    }

    #[test]
    fn day03p2() {
        let input = std::fs::read_to_string("input-test/day03.txt").unwrap();
        assert_eq!(super::day03p2::puzzle_2(input), 467835);
    }

    #[test]
    fn day04p1() {
        let input = std::fs::read_to_string("input-test/day04.txt").unwrap();
        assert_eq!(super::day04p1::puzzle_1(input), 13);
    }

    #[test]
    fn day04p2() {
        let input = std::fs::read_to_string("input-test/day04.txt").unwrap();
        assert_eq!(super::day04p2::puzzle_2(input), 30);
    }

    #[test]
    fn day05p1() {
        let input = std::fs::read_to_string("input-test/day05.txt").unwrap();
        assert_eq!(super::day05::puzzle_1(input), 35);
    }

    #[test]
    fn day05p2() {
        let input = std::fs::read_to_string("input-test/day05.txt").unwrap();
        assert_eq!(super::day05::puzzle_2(input), 46);
    }

    #[test]
    fn day06p1() {
        let input = std::fs::read_to_string("input-test/day06.txt").unwrap();
        assert_eq!(super::day06::puzzle_1(input), 288);
    }

    #[test]
    fn day06p2() {
        let input = std::fs::read_to_string("input-test/day06.txt").unwrap();
        assert_eq!(super::day06::puzzle_2(input), 71503);
    }

    #[test]
    fn day07p1() {
        let input = std::fs::read_to_string("input-test/day07.txt").unwrap();
        assert_eq!(super::day07::puzzle_1(input), 6440);
    }

    #[test]
    fn day07p2() {
        let input = std::fs::read_to_string("input-test/day07.txt").unwrap();
        assert_eq!(super::day07::puzzle_2(input), 5905);
    }

    #[test]
    fn day08p1() {
        let input = std::fs::read_to_string("input-test/day08p1.txt").unwrap();
        assert_eq!(super::day08::puzzle_1(input), 6);
    }

    #[test]
    fn day08p2() {
        let input = std::fs::read_to_string("input-test/day08p2.txt").unwrap();
        assert_eq!(super::day08::puzzle_2(input), 6);
    }
}