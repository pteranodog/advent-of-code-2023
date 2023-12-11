fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {
    let input_lines = read_lines(input);
    let mut universe = parse_input(input_lines);
    inflate_space(&mut universe, 2);
    let mut sum = 0;
    for galaxy_1_index in 0..universe.galaxies.len() {
        for galaxy_2_index in galaxy_1_index..universe.galaxies.len() {
            sum += universe.galaxies[galaxy_1_index].position.distance_to(&universe.galaxies[galaxy_2_index].position);
        }
    }
    sum as i32
}

pub fn puzzle_2(input: String) -> i128 {
    let input_lines = read_lines(input);
    let mut universe = parse_input(input_lines);
    inflate_space(&mut universe, 1000000);
    let mut sum: i128 = 0;
    for galaxy_1_index in 0..universe.galaxies.len() {
        for galaxy_2_index in galaxy_1_index..universe.galaxies.len() {
            sum += universe.galaxies[galaxy_1_index].position.distance_to(&universe.galaxies[galaxy_2_index].position) as i128;
        }
    }
    sum
}

fn parse_input(input: Vec<String>) -> Universe {
    let mut universe = Universe {
        galaxies: Vec::new(),
        size: Vector {
            x: input[0].len(),
            y: input.len(),
        },
    };
    for (y_position, line) in input.iter().enumerate() {
        for (x_position, character) in line.chars().enumerate() {
            if character == '#' {
                universe.galaxies.push(Galaxy {
                    position: Vector {
                        x: x_position,
                        y: y_position,
                    },
                });
            }
        }
    }
    universe
}

struct Universe {
    galaxies: Vec<Galaxy>,
    size: Vector,
}

#[derive(Clone)]
struct Galaxy {
    position: Vector,
}

#[derive(Clone)]
struct Vector {
    x: usize,
    y: usize,
}

impl Vector {
    fn distance_to(&self, other: &Vector) -> i128 {
        ((self.x as i128 - other.x as i128).abs() + (self.y as i128 - other.y as i128).abs()) as i128
    }
}

fn inflate_space(universe: &mut Universe, inflation_factor: usize) {
    let inflation_factor = inflation_factor - 1;
    let mut inflation_factor_x = 0;
    let mut inflation_factor_y = 0;
    let mut galaxies_inflated_x = Vec::new();
    for x_index in 0..universe.size.x {
        let mut galaxies_with_x = universe.galaxies.iter().filter(|g| g.position.x == x_index).map(|g| g.clone()).collect::<Vec<Galaxy>>();
        if galaxies_with_x.len() > 0 {
            for galaxy_index in 0..galaxies_with_x.len() {
                galaxies_with_x[galaxy_index].position.x += inflation_factor_x;
                galaxies_inflated_x.push(galaxies_with_x[galaxy_index].clone());
            }
        } else {
            inflation_factor_x += inflation_factor;
        }
    }
    let mut galaxies_inflated_x_and_y = Vec::new();
    for y_index in 0..universe.size.y {
        let mut galaxies_with_y = galaxies_inflated_x.iter().filter(|g| g.position.y == y_index).map(|g| g.clone()).collect::<Vec<Galaxy>>();
        if galaxies_with_y.len() > 0 {
            for galaxy_index in 0..galaxies_with_y.len() {
                galaxies_with_y[galaxy_index].position.y += inflation_factor_y;
                galaxies_inflated_x_and_y.push(galaxies_with_y[galaxy_index].clone());
            }
        } else {
            inflation_factor_y += inflation_factor;
        }
    }    
    universe.galaxies = galaxies_inflated_x_and_y;
}