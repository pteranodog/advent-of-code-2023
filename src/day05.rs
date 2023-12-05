fn read_lines(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn puzzle_1(input: String) -> u64 {
    let input_text = read_lines(input);

    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    parse_input(input_text, &mut seeds, &mut maps);

    let mut locations: Vec<u64> = Vec::new();
    for seed in seeds {
        locations.push(map_start_to_finish(seed, &maps));
    }

    locations.sort();
    locations[0] as u64
}

pub fn puzzle_2(input: String) -> u64 {
    let input_text = read_lines(input);

    let mut seeds = Vec::new();
    let mut maps = Vec::new();

    parse_input(input_text, &mut seeds, &mut maps);

    let mut min_location = map_start_to_finish(seeds[0], &maps);

    let mut index = 0;
    while index < seeds.len() {
        let stop_seed = seeds[index] + seeds[index + 1];
        let mut seed_number = seeds[index];
        while seed_number < stop_seed {
            let location;
            let max_range;
            (location, max_range) = map_start_to_finish_max_range(seed_number, &maps);
            if location < min_location {
                min_location = location;
            }
            if seed_number + max_range > stop_seed {
                seed_number = stop_seed;
            }
            else {
                seed_number += max_range;
            }
        }
        index += 2;
    }

    min_location
}

#[derive(Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    range: u64,
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn new() -> Map {
        Map {
            ranges: Vec::new(),
        }
    }

    fn add_range(&mut self, destination_start: u64, source_start: u64, range: u64) {
        self.ranges.push(MapRange {
            destination_start,
            source_start,
            range,
        });
    }

    fn map(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if source >= range.source_start && source < range.source_start + range.range {
                return range.destination_start + (source - range.source_start);
            }
        }
        source
    }

    fn map_max_range(&self, source: u64) -> (u64, u64) {
        let mut remaining_range = u64::MAX;
        for range in &self.ranges {
            if source >= range.source_start && source < range.source_start + range.range {
                let destination = range.destination_start + (source - range.source_start);
                if range.range - (source - range.source_start) < remaining_range {
                    remaining_range = range.range - (source - range.source_start);
                }
                return (destination, remaining_range);
            }
            else if source < range.source_start {
                if range.source_start - source < remaining_range {
                    remaining_range = range.source_start - source;
                }
            }
        }
        (source, remaining_range)
    }
}

fn parse_input(input: Vec<String>, seeds: &mut Vec<u64>, maps: &mut Vec<Map>) {
    let seed_strings: Vec<&str> = input[0].split(":").collect::<Vec<&str>>()[1].trim().split_whitespace().collect();
    for seed in seed_strings {
        seeds.push(seed.parse::<u64>().unwrap());
    }
    let mut index = 2;
    while index < input.len() {
        let mut map = Map::new();
        index += 1;
        while index < input.len() && input[index] != "" {
            let new_range = input[index].split_whitespace().collect::<Vec<&str>>();
            let destination_start = new_range[0].parse::<u64>().unwrap();
            let source_start = new_range[1].parse::<u64>().unwrap();
            let range = new_range[2].parse::<u64>().unwrap();
            map.add_range(destination_start, source_start, range);
            index += 1;
        }
        maps.push(map);
        index += 1;
    }
}

fn map_start_to_finish(start: u64, maps: &Vec<Map>) -> u64 {
    let mut result = start;
    for map in maps {
        result = map.map(result);
    }
    result
}

fn map_start_to_finish_max_range(start: u64, maps: &Vec<Map>) -> (u64, u64) {
    let mut result = start;
    let mut max_range;
    let mut actual_max_range = u64::MAX;
    for map in maps {
        (result, max_range) = map.map_max_range(result);
        if max_range < actual_max_range {
            actual_max_range = max_range;
        }
    }
    (result, actual_max_range)
}