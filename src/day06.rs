fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {

    let times: Vec<i32> = vec![48, 98, 90, 83];
    let dists: Vec<i32> = vec![390, 1103, 1112, 1360];

    //let times: Vec<i32> = vec![7, 15, 30];
    //let dists: Vec<i32> = vec![9, 40, 200];

    let times: Vec<u64> = vec![48989083];
    let dists: Vec<u64> = vec![390110311121360];
    
    let mut sum = 0;
    for index in 0..1 {
        let mut final_dists: Vec<i32> = Vec::new();
        for hold_time in 0..times[index] {
            let new_dist = (times[index] - hold_time) * hold_time;
            if new_dist > dists[index] {
                sum += 1;
            }
        }

    }

    sum as i32
}

pub fn puzzle_2(input: String) -> i32 {



    0
}

