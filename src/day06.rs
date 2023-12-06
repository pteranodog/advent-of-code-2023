fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {

    let times;
    let dists;
    (times, dists) = parse_input(read_lines(input));
    
    let mut product = 1;
    for index in 0..times.len() {
        let mut record_distances = Vec::new();
        for hold_time in 0..times[index] {
            let new_dist = (times[index] - hold_time) * hold_time;
            if new_dist > dists[index] {
                record_distances.push(new_dist);
            }
        }
        product *= record_distances.len();
    }

    product as i32
}

pub fn puzzle_2(input: String) -> i32 {

    let times;
    let dists;
    (times, dists) = parse_input(read_lines(input));
    let time;
    let dist;
    (time, dist) = convert_to_pt_2(times, dists);

    let mut sum = 0;
    for hold_time in 0..time {
        let new_dist = (time - hold_time) * hold_time;
        if new_dist > dist {
            sum += 1;
        }
    }

    sum as i32
}

fn parse_input(input: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut times = Vec::new();
    let mut dists = Vec::new();

    for number in input[0].split_whitespace().skip(1) {
        times.push(number.parse().unwrap());
    }
    for number in input[1].split_whitespace().skip(1) {
        dists.push(number.parse().unwrap());
    }

    (times, dists)
}

fn convert_to_pt_2(times: Vec<i32>, dists: Vec<i32>) -> (u64, u64) {
    let mut times_string: String = String::new();
    let mut dists_string: String = String::new();

    for time in times {
        times_string.push_str(&time.to_string());
    }
    for dist in dists {
        dists_string.push_str(&dist.to_string());
    }

    (times_string.parse().unwrap(), dists_string.parse().unwrap())
}