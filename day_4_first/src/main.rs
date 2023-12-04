use std::fs::read_to_string;

const FILE_PATH: &str = "input.txt";

fn main() {
    let input = read_lines(FILE_PATH);

    let mut cards = Vec::new();
    for line in input {
        cards.push(parse_line_to_card(line));
    }

    let mut sum = 0;

    for card in cards {
        let mut card_exponent = 0;
        for i in card.values {
            for j in card.winning.clone() {
                if i == j {
                    card_exponent = card_exponent + 1;
                }
            }
        }
        if card_exponent > 0 {
            sum = sum + 1 * 2_i32.pow(card_exponent - 1);
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

#[derive(Debug)]
struct Card {
    winning: Vec<i32>,
    values: Vec<i32>,
}

fn parse_line_to_card(line: String) -> Card {
    let mut return_card = Card {
        winning: Vec::new(),
        values: Vec::new(),
    };

    let winning_nums = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[0].trim().split_whitespace().collect::<Vec<&str>>();
    for num in winning_nums {
        return_card.winning.push(num.parse::<i32>().unwrap());
    }
    let values = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>();
    for num in values {
        return_card.values.push(num.parse::<i32>().unwrap());
    }

    return_card
}