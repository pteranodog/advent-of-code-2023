use num;

fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {
    let input_lines = read_lines(input);
    let (directions, nodes) = parse_input(input_lines);
    
    let mut current_node = "AAA".to_string();
    let mut steps = 0;
    while current_node != "ZZZ" {
        let direction = &directions[steps % directions.len()];
        let node = nodes.iter().find(|n| n.name == current_node).unwrap();
        current_node = get_next_node_name(&node, &direction).to_string();
        steps += 1;
    }

    steps as i32
}

pub fn puzzle_2(input: String) -> i128 {
    let input_lines = read_lines(input);
    let (directions, nodes) = parse_input(input_lines);

    let mut current_nodes = nodes.iter().filter(|n|
        n.name.chars().nth(2).unwrap() == 'A'
    ).collect::<Vec<&Node>>();

    let mut z_nodes = Vec::new();
    for node in nodes.iter().filter(|n| n.name.chars().nth(2).unwrap() == 'Z') {
        z_nodes.push(ZNode {
            name: node.name.clone(),
            last_found: None,
            cycle: None,
        });
    }

    let mut steps = 0;
    let mut checked_z_nodes = Vec::new();
    while z_nodes.iter().any(|n| n.cycle.is_none()) {
        let direction = &directions[steps % directions.len()];
        for index in 0..current_nodes.len() {
            current_nodes[index] = &nodes[get_next_node_index(&current_nodes[index], &direction)];
            if current_nodes[index].name.chars().nth(2).unwrap() == 'Z' {
                let matching_node = z_nodes.iter_mut().find(|n| n.name == current_nodes[index].name).unwrap();
                if matching_node.last_found.is_none() {
                    matching_node.last_found = Some(steps);
                } else {
                    matching_node.cycle = Some(steps - matching_node.last_found.unwrap());
                    matching_node.last_found = Some(steps);
                }
                checked_z_nodes.push(current_nodes[index].name.clone());
            }
        }
        steps += 1;
    }
    let mut least_common_multiple: u128 = z_nodes.iter().next().unwrap().cycle.unwrap().clone() as u128;
    for znode in z_nodes.iter().skip(1) {
        least_common_multiple = num::Integer::lcm(&least_common_multiple, &(znode.cycle.unwrap().clone() as u128));
    }

    least_common_multiple.try_into().unwrap()
}

fn parse_input(input: Vec<String>) -> (Vec<Direction>, Vec<Node>) {
    let mut nodes = Vec::new();

    let directions: Vec<Direction> = input.iter().next().unwrap().chars().map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    }).collect();

    let lines = input.iter().skip(2);

    for line in lines {
        let name = &line.split_whitespace().next().unwrap().to_string();
        let left = &line.split_whitespace().nth(2).unwrap().to_string()[1..4];
        let right = &line.split_whitespace().nth(3).unwrap().to_string()[..3];
        nodes.push(Node {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            index_left: 0,
            index_right: 0,
        });
    }

    for index in 0..nodes.len() {
        nodes[index].index_left = nodes.iter().position(|n| n.name == nodes[index].left).unwrap();
        nodes[index].index_right = nodes.iter().position(|n| n.name == nodes[index].right).unwrap();
    }

    (directions, nodes)
}

struct Node {
    name: String,
    left: String,
    right: String,
    index_left: usize,
    index_right: usize,
}

enum Direction {
    Left,
    Right,
}

fn get_next_node_name(current_node: &Node, direction: &Direction) -> String {
    match direction {
        Direction::Left => current_node.left.clone(),
        Direction::Right => current_node.right.clone(),
    }
}

fn get_next_node_index(current_node: &Node, direction: &Direction) -> usize {
    match direction {
        Direction::Left => current_node.index_left,
        Direction::Right => current_node.index_right,
    }
}

struct ZNode {
    name: String,
    last_found: Option<usize>,
    cycle: Option<usize>,
}