const VALID_DIGITS: [&str; 18] = ["one","two","three","four","five","six","seven","eight","nine","1","2","3","4","5","6","7","8","9"];

pub fn puzzle_2(input: String) -> i32 {
    let mut sum = 0;

    let input_text = read_lines(input);

    for line in input_text {
        sum += 10 * find_first_digit(&line) + find_last_digit(&line);        
    }

    sum as i32
}

fn read_lines(input: String) -> Vec<String> {
    input
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