pub fn puzzle_2(input_line: String) -> i32 {
    let input_text = read_lines(input_line);
    let all_games = parse_input(input_text);
    let game_powers = find_game_powers(all_games);
    let game_id_sum = sum_vector(game_powers);

    game_id_sum
}

fn read_lines(input: String) -> Vec<String> {
    input
        .lines()  // split the string into an iterat?or of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn sum_vector(vector_to_sum: Vec<i32>) -> i32 {
    vector_to_sum.iter().sum()
}

fn find_game_powers(game_list: Vec<Vec<i32>>) -> Vec<i32> {
    let mut game_powers: Vec<i32> = Vec::new();

    for index in 0..game_list.len() {
        game_powers.push(game_list[index][0] * game_list[index][1] * game_list[index][2])
    }

    game_powers
}

fn parse_input(full_input: Vec<String>) -> Vec<Vec<i32>> {
    let mut return_value: Vec<Vec<i32>> = Vec::new();

    for input_line in full_input {
        return_value.push(parse_input_line(input_line));
    }

    return_value
}

fn parse_input_line(input_line: String) -> Vec<i32> {
    let split_input_line: Vec<&str> = input_line.split(":").collect();
    let right_side = split_input_line[1].trim();
    let sublines: Vec<&str> = right_side.split(";").collect();
    let mut parsed_input_line = vec![0,0,0];

    for subline in sublines {
        let subline_results = parse_input_subline(subline.to_string());
        if subline_results[0] > parsed_input_line[0] {
            parsed_input_line[0] = subline_results[0];
        }
        if subline_results[1] > parsed_input_line[1] {
            parsed_input_line[1] = subline_results[1];
        }
        if subline_results[2] > parsed_input_line[2] {
            parsed_input_line[2] = subline_results[2];
        }
    }

    parsed_input_line
}

fn parse_input_subline(input_subline: String) -> Vec<i32> {
    let input_subline = input_subline.trim();
    let split_subline: Vec<&str> = input_subline.split(",").collect();
    let mut parsed_subline = vec![0,0,0];

    for subsubline in split_subline {
        let split_subsubline: Vec<&str> = subsubline.split_whitespace().collect();
        if split_subsubline[1] == "red" {
            parsed_subline[0] = split_subsubline[0].parse().expect("Number not present.");
        }
        if split_subsubline[1] == "green" {
            parsed_subline[1] = split_subsubline[0].parse().expect("Number not present.");
        }
        if split_subsubline[1] == "blue" {
            parsed_subline[2] = split_subsubline[0].parse().expect("Number not present.");
        }
    }

    parsed_subline
}
