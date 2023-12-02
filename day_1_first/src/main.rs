use std::fs::read_to_string;

const FILE_PATH: &str = "input.txt";

fn main() {
    let mut sum = 0;

    let input_text = read_lines(FILE_PATH);

    for line in input_text {
        let characters = line.chars();
        let reverse_characters = characters.clone().rev();
        
        for character in characters {
            if character.is_ascii_digit() {
                let digit = character.to_digit(10);
                let digit = match digit {
                    Some(x) => x,
                    None    => 0,
                };
                sum += 10 * digit;
                break;
            }
        }
        
        for character in reverse_characters {
            if character.is_ascii_digit() {
                let digit = character.to_digit(10);
                let digit = match digit {
                    Some(x) => x,
                    None    => 0,
                };
                sum += digit;
                break;
            }
        }
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