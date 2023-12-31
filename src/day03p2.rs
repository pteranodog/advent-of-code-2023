pub fn puzzle_2(input_line: String) -> i32 {
    let input = read_lines(input_line);

    let all_numbers = get_all_numbers(input.clone());

    let all_symbols = get_all_symbols(input.clone());

    let all_symbols = find_nums_with_adj_symbols(all_numbers, all_symbols);

    let gears = get_gears(all_symbols);

    let result = sum_ratios(gears);

    result
}

#[derive(Debug)]
struct Number1D {
    value: i32,
    length: i32,
    location: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Number2D {
    value: i32,
    length: i32,
    xlocation: i32,
    ylocation: i32,
}

#[derive(Debug)]
struct Symbol1D {
    location: i32,
}

#[derive(Debug)]
#[derive(Clone)]
struct Symbol2D {
    xlocation: i32,
    ylocation: i32,
    neighbors: Vec<Number2D>,
}

fn read_lines(input: String) -> Vec<String> {
    input
        .lines()  // split the string into an iterat?or of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_numbers_from_line(line: String) -> Vec<Number1D> {
    let mut result = Vec::new();
    let char_array: Vec<char> = line.chars().collect();

    let mut index = 0;
    while index < line.len() {
        if ! char_array[index].is_ascii_digit() {
            index += 1;
            continue;
        }
        
        let location = index as i32;
        let mut length = 1;
        let mut number: i32 = char_array[index].to_digit(10).expect("Character was not an ASCII digit.") as i32;
        while index + 1 < char_array.len() && char_array[index + 1].is_ascii_digit() {
            number = 10 * number + char_array[index + 1].to_digit(10).expect("Character was not an ASCII digit.") as i32;
            length += 1;
            index += 1;
        }

        let found_number = Number1D {
            value: number,
            length: length,
            location: location,
        };

        result.push(found_number);
        index += 1;
    }

    result
}

fn get_all_numbers(input: Vec<String>) -> Vec<Number2D> {
    let mut all_numbers = Vec::new();

    for index in 0..input.len() as i32 {
        let line = &input[index as usize];
        let numbers_from_line = get_numbers_from_line(line.to_string());
        for number in numbers_from_line {
            let full_number = expand_number_2_d(number, index);
            all_numbers.push(full_number);
        }
    }

    all_numbers
}

fn get_symbols_from_line(line: String) -> Vec<Symbol1D> {
    let mut result = Vec::new();
    let char_array: Vec<char> = line.chars().collect();

    for index in 0..line.len() as i32 {
        let character = char_array[index as usize];
        if character == '*' {
            result.push(Symbol1D {
                location: index,
            })
        }
    }

    result
}

fn get_all_symbols(input: Vec<String>) -> Vec<Symbol2D> {
    let mut all_symbols = Vec::new();

    for index in 0..input.len() as i32 {
        let line = &input[index as usize];
        let symbols_from_line = get_symbols_from_line(line.to_string());
        for symbol in symbols_from_line {
            let full_symbol = expand_symbol_2_d(symbol, index);
            all_symbols.push(full_symbol);
        }
    }

    all_symbols
}

fn expand_number_2_d(original_number: Number1D, new_y: i32) -> Number2D {
    Number2D {
        value: original_number.value,
        length: original_number.length,
        xlocation: original_number.location,
        ylocation: new_y,
    }
}

fn expand_symbol_2_d(original_symbol: Symbol1D, new_y: i32) -> Symbol2D {
    Symbol2D {
        xlocation: original_symbol.location,
        ylocation: new_y,
        neighbors: Vec::new(),
    }
}

fn find_nums_with_adj_symbols(numbers: Vec<Number2D>, mut symbols: Vec<Symbol2D>) -> Vec<Symbol2D> {    
    for number in numbers {
        for symbol in &mut symbols {
            let min_x = number.xlocation - 1;
            let max_x = number.xlocation + number.length;
            let min_y = number.ylocation - 1;
            let max_y = number.ylocation + 1;

            if symbol_within_bounds(symbol, min_x, max_x, min_y, max_y) {
                symbol.neighbors.push(number.clone());
                break;
            }
        }
    }

    symbols
}

fn symbol_within_bounds(symbol: &Symbol2D, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> bool {
    symbol.xlocation >= min_x && symbol.xlocation <= max_x && symbol.ylocation >= min_y && symbol.ylocation <= max_y
}

fn get_gears(symbols: Vec<Symbol2D>) -> Vec<Symbol2D> {
    let mut all_gears = Vec::new();

    for symbol in symbols {
        if symbol.neighbors.len() == 2 {
            all_gears.push(symbol.clone());
        }
    }

    all_gears
}

fn sum_ratios(gears: Vec<Symbol2D>) -> i32 {
    let mut sum = 0;

    for gear in gears {
        sum += gear.neighbors[0].value * gear.neighbors[1].value;
    }

    sum
}