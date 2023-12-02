use std::fs::read_to_string;

const FILE_PATH: &str = "input.txt";
const VALID_DIGITS: [&str; 18] = ["one","two","three","four","five","six","seven","eight","nine","1","2","3","4","5","6","7","8","9"];

fn main() {
    let mut sum = 0;

    let input_text = read_lines(FILE_PATH);

    for line in input_text {
        sum += 10 * find_first_digit(&line) + find_last_digit(&line);        
    }

    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterat?or of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn find_first_digit(string_to_match: &String) -> usize {
    let mut digit_positions: Vec<Option<usize>> = Vec::new();

    for digit in VALID_DIGITS {
        digit_positions.push(string_to_match.find(digit));
    }

    let mut min_position = 0;

    while digit_positions[min_position].is_none() {
        min_position += 1;
    }

    for position in 1..digit_positions.len() {
        if digit_positions[position].is_some_and(|x| x < digit_positions[min_position].expect("Panic? idk")) {
            min_position = position;
        }
    }

    min_position % 9 + 1
}

fn find_last_digit(string_to_match: &String) -> usize {
    let mut digit_positions: Vec<Option<usize>> = Vec::new();

    for digit in VALID_DIGITS {
        digit_positions.push(string_to_match.rfind(digit));
    }

    let mut max_position = 0;
    for position in 1..digit_positions.len() {
        if &digit_positions[position] > &digit_positions[max_position] {
            max_position = position;
        }
    }

    max_position % 9 + 1
}