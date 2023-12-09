fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {
    let input_lines = read_lines(input);
    let data = parse_input(input_lines);
    let mut sum = 0;
    for data_line in data {
        let mut derivatives = collect_derivatives(data_line);
        let mut next_term = 0;
        while derivatives.len() != 0 {
            next_term += derivatives.pop().unwrap().last().unwrap();
        }
        sum += next_term;
    }

    sum
}

pub fn puzzle_2(input: String) -> i32 {
    let input_lines = read_lines(input);
    let data = parse_input(input_lines);
    let mut sum = 0;
    for data_line in data {
        let mut derivatives = collect_derivatives(data_line);
        let mut next_term = 0;
        while derivatives.len() != 0 {
            next_term = derivatives.pop().unwrap()[0] - next_term;
        }
        sum += next_term;
    }

    sum
}

fn parse_input(input: Vec<String>) -> Vec<Vec<i32>> {
    let mut output = Vec::new();
    for line in input {
        let mut line_vec = Vec::new();
        for number in line.split_whitespace() {
            line_vec.push(number.parse().unwrap());
        }
        output.push(line_vec);
    }
    output
}

fn collect_derivatives(data: Vec<i32>) -> Vec<Vec<i32>> {
    let mut layer = 0;
    let mut derivatives = Vec::new();
    derivatives.push(data.clone());
    while !derivatives[layer].iter().all(|d| *d == 0) {
        let mut next_layer = Vec::new();
        for index in 1..derivatives[layer].len() {
            next_layer.push(derivatives[layer][index] - derivatives[layer][index - 1]);
        }
        derivatives.push(next_layer);
        layer += 1;
    }
    derivatives
}