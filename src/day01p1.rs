pub fn puzzle_1(input: String) -> i32 {
    let mut sum = 0;

    let input_text = read_lines(input);

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

    sum as i32
}

fn read_lines(input: String) -> Vec<String> {
    input 
        .lines()  // split the string into an iterat?or of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}