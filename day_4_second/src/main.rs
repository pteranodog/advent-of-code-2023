use std::fs::read_to_string;

const FILE_PATH: &str = "input.txt";

fn main() {
    let input = read_lines(FILE_PATH);

    let mut cards = Vec::new();

    for index in 0..input.len() {
        cards.push(parse_line_to_card(input[index].clone()));
    }

    for index in 0..cards.len() {
        if cards[index].matches > 0 {
            for match_count in 1..cards[index].matches + 1 {
                cards[index + match_count as usize].count += cards[index].count;
            }
        }
    }
    
    let mut card_count = 0;
    for card in cards {
        card_count += card.count;
    }
    
    println!("{}", card_count);
    
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterat?or of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    winning: Vec<i32>,
    values: Vec<i32>,
    matches: i32,
    count: i32,
}

fn parse_line_to_card(line: String) -> Card {
    let mut return_card = Card {
        winning: Vec::new(),
        values: Vec::new(),
        matches: 0,
        count: 1,
    };

    let winning_nums = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[0].trim().split_whitespace().collect::<Vec<&str>>();
    for num in winning_nums {
        return_card.winning.push(num.parse::<i32>().unwrap());
    }
    let values = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>()[1].trim().split_whitespace().collect::<Vec<&str>>();
    for num in values {
        return_card.values.push(num.parse::<i32>().unwrap());
    }

    for value in return_card.values.clone() {
        if return_card.winning.contains(&value) {
            return_card.matches += 1;
        }
    }

    return_card
}