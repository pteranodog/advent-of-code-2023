fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> i32 {
    let input_lines = read_lines(input);
    let mut input = parse_input(input_lines);
    let start = find_start(&input);
    let starting_pipe = get_starting_pipe(&input, &start);
    input[start.1][start.0].distance_from_start = Some(0);
    input[start.1][start.0].pipe = starting_pipe;

    for _ in 0..input.len() * input[0].len() {
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                set_distance_for_neighbors(&mut input, &(x, y));
            }
        }
    }

    let all_tiles = input.iter().flatten().filter(|n| n.distance_from_start.is_some()).collect::<Vec<&Tile>>();
    all_tiles.iter().map(|t| t.distance_from_start.unwrap()).max().unwrap()
}

pub fn puzzle_2(input: String) -> i32 {
    let input_lines = read_lines(input);
    let mut input = parse_input(input_lines);
    let start = find_start(&input);
    let starting_pipe = get_starting_pipe(&input, &start);
    input[start.1][start.0].distance_from_start = Some(0);
    input[start.1][start.0].pipe = starting_pipe;

    for _ in 0..input.len() * input[0].len() {
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                set_distance_for_neighbors(&mut input, &(x, y));
            }
        }
    }
    
    for y in 0..input.len() {
        let mut currently_inside_loop = false;
        let mut current_corner: Option<Pipe> = None;
        for x in 0..input[0].len() {
            if input[y][x].distance_from_start.is_some() {
                let current_pipe = input[y][x].pipe.clone();
                if current_pipe == Pipe::NorthSouth {
                    currently_inside_loop = !currently_inside_loop;
                } else if current_pipe.is_corner() {
                    if current_corner.is_some() && current_pipe == current_corner.clone().unwrap().opposing_corner() {
                        currently_inside_loop = !currently_inside_loop;
                        current_corner = None;
                    } else if current_corner.clone().is_some() {
                        current_corner = None;
                    } else {
                        current_corner = Some(current_pipe.clone());
                    }
                }
            } else {
                input[y][x].inside_loop = currently_inside_loop;
            }
        }
    }
    
    input.iter().flatten().filter(|n| n.inside_loop).collect::<Vec<&Tile>>().len() as i32
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Tile>> {
    let mut output = Vec::new();
    for line in input {
        let mut line_vec = Vec::new();
        for character in line.chars() {
            line_vec.push(Tile {
                distance_from_start: None,
                pipe: match character {
                    '|' => Pipe::NorthSouth,
                    '-' => Pipe::EastWest,
                    'L' => Pipe::NorthEast,
                    'J' => Pipe::NorthWest,
                    'F' => Pipe::SouthEast,
                    '7' => Pipe::SouthWest,
                    '.' => Pipe::NoPipe,
                    'S' => Pipe::Start,
                    _ => panic!("Invalid input character"),
                },
                inside_loop: false,
            });
        }
        output.push(line_vec);
    }
    output
}

fn find_start(input: &Vec<Vec<Tile>>) -> (usize, usize) {
    for (y, line) in input.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile.pipe == Pipe::Start {
                return (x, y);
            }
        }
    }
    panic!("No start found");
}

fn get_starting_pipe(input: &Vec<Vec<Tile>>, start: &(usize, usize)) -> Pipe {
    let mut north = false;
    let mut east = false;
    let mut south = false;
    let mut west = false;
    if start.1 < input.len()
        && (input[start.1 + 1][start.0].pipe == Pipe::NorthSouth
        || input[start.1 + 1][start.0].pipe == Pipe::NorthEast
        || input[start.1 + 1][start.0].pipe == Pipe::NorthWest) {
        south = true;
    }
    if start.1 > 0
        && (input[start.1 - 1][start.0].pipe == Pipe::NorthSouth
        || input[start.1 - 1][start.0].pipe == Pipe::SouthEast
        || input[start.1 - 1][start.0].pipe == Pipe::SouthWest) {
        north = true;
    }
    if start.0 < input[0].len()
        && (input[start.1][start.0 + 1].pipe == Pipe::EastWest
        || input[start.1][start.0 + 1].pipe == Pipe::NorthWest
        || input[start.1][start.0 + 1].pipe == Pipe::SouthWest) {
        east = true;
    }
    if start.0 > 0
        && (input[start.1][start.0 - 1].pipe == Pipe::EastWest
        || input[start.1][start.0 - 1].pipe == Pipe::NorthEast
        || input[start.1][start.0 - 1].pipe == Pipe::SouthEast) {
        west = true;
    }
    match (north, east, south, west) {
        (true, false, true, false) => Pipe::NorthSouth,
        (false, true, false, true) => Pipe::EastWest,
        (true, true, false, false) => Pipe::NorthEast,
        (true, false, false, true) => Pipe::NorthWest,
        (false, true, true, false) => Pipe::SouthEast,
        (false, false, true, true) => Pipe::SouthWest,
        _ => panic!("Invalid start"),
    }
}

fn set_distance_for_neighbors(input: &mut Vec<Vec<Tile>>, tile_index: &(usize, usize)) {
    if input[tile_index.1][tile_index.0].distance_from_start.is_none() {
        return;
    }
    let neighbors = input[tile_index.1][tile_index.0].pipe.valid_neighbors();
    let next_tile_1 = (tile_index.0 as i32 + neighbors.0.0, tile_index.1 as i32 + neighbors.0.1);
    let next_tile_2 = (tile_index.0 as i32 + neighbors.1.0, tile_index.1 as i32 + neighbors.1.1);
    if input[next_tile_1.1 as usize][next_tile_1.0 as usize].pipe != Pipe::NoPipe {
        if input[next_tile_1.1 as usize][next_tile_1.0 as usize].distance_from_start == None {
            input[next_tile_1.1 as usize][next_tile_1.0 as usize].distance_from_start = Some(input[tile_index.1][tile_index.0].distance_from_start.unwrap() + 1);
        }
    }
    if input[next_tile_2.1 as usize][next_tile_2.0 as usize].pipe != Pipe::NoPipe {
        if input[next_tile_2.1 as usize][next_tile_2.0 as usize].distance_from_start == None {
            input[next_tile_2.1 as usize][next_tile_2.0 as usize].distance_from_start = Some(input[tile_index.1][tile_index.0].distance_from_start.unwrap() + 1);
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    NoPipe,
    Start,
}

impl Pipe {
    fn valid_neighbors(&self) -> ((i32, i32), (i32, i32)) {
        match self {
            Pipe::NorthSouth => ((0, -1), (0, 1)),
            Pipe::EastWest => ((1, 0), (-1, 0)),
            Pipe::NorthEast => ((0, -1), (1, 0)),
            Pipe::NorthWest => ((0, -1), (-1, 0)),
            Pipe::SouthEast => ((0, 1), (1, 0)),
            Pipe::SouthWest => ((0, 1), (-1, 0)),
            _ => panic!("Can't match neighbors for this pipe"),
        
        }
    }

    fn is_corner(&self) -> bool {
        match self {
            Pipe::NorthEast => true,
            Pipe::NorthWest => true,
            Pipe::SouthEast => true,
            Pipe::SouthWest => true,
            _ => false,
        }
    }

    fn opposing_corner(&self) -> Pipe {
        match self {
            Pipe::NorthEast => Pipe::SouthWest,
            Pipe::NorthWest => Pipe::SouthEast,
            Pipe::SouthEast => Pipe::NorthWest,
            Pipe::SouthWest => Pipe::NorthEast,
            _ => panic!("Can't get opposing corner for this pipe"),
        }
    }
}

#[derive(Debug)]
struct Tile {
    distance_from_start: Option<i32>,
    pipe: Pipe,
    inside_loop: bool,
}