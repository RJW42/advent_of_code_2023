use crate::days::Part;
use crate::days::{read_lines, parse_num};

use std::fs::File;
use std::io::{self};
use std::cmp::Ordering;

struct Input {
    seeds: Vec<u64>,
    seed_to_soil: Vec<MapElement>,
    soil_to_fertilizer: Vec<MapElement>,
    fertilizer_to_water: Vec<MapElement>,
    water_to_light: Vec<MapElement>,
    light_to_temerature: Vec<MapElement>,
    temerature_to_humidity: Vec<MapElement>,
    humidity_to_location: Vec<MapElement>
}

#[derive(Eq)]
struct MapElement {
    des_range_start: u64,
    src_range_start: u64,
    length: u64
}


pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => todo!(),
    }
}



fn part1(file_name: &str) -> Result<u32, &'static str> {
    let input = parse_input(file_name)?;

    print_input(&input);
    let mut min = std::u64::MAX;
    for s in &input.seeds {
        let n = get_soil_number(&input, *s);
        println!("s{} n{}", *s, n);
        if n < min {
            min = n;
        }
    }

    Ok(min as u32)
}


fn get_soil_number(input: &Input, seed: u64) -> u64 {
    let soil = get_mapping(seed, &input.seed_to_soil);
    let fert = get_mapping(soil, &input.soil_to_fertilizer);
    let water = get_mapping(fert, &input.fertilizer_to_water);
    let light = get_mapping(water, &input.water_to_light);
    let temp = get_mapping(light, &input.light_to_temerature);
    let hum = get_mapping(temp, &input.temerature_to_humidity);
    let loc = get_mapping(hum, &input.humidity_to_location);
    return loc;
}

fn get_mapping(n: u64, map: &Vec<MapElement>) -> u64 {
    for el in map {
        if n < el.src_range_start {
            return n;
        }

        if n < el.src_range_start + el.length {
            return el.des_range_start + (n - el.src_range_start)
        }
    }

    return n;
}


impl Ord for MapElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.src_range_start.cmp(&other.src_range_start)
    }
}

impl PartialOrd for MapElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapElement {
    fn eq(&self, other: &Self) -> bool {
        self.src_range_start == other.src_range_start
    }
}

fn parse_input(file_name: &str) -> Result<Input, &'static str> {
    let Ok(mut lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let Some(seeds) = parse_seeds(&mut lines) else {
        return Err("Failed to parse seeds");
    };

    let Some(seed_to_soil) = parse_map(&mut lines) else {
        return Err("Failed to parse soil to soil");
    };

    let Some(soil_to_fertilizer) = parse_map(&mut lines) else {
        return Err("Failed to parse soil_to_fertilizer");
    };

    let Some(fertilizer_to_water) = parse_map(&mut lines) else {
        return Err("Failed to parse fertilizer_to_water");
    };

    let Some(water_to_light) = parse_map(&mut lines) else {
        return Err("Failed to parse water_to_light");
    };

    let Some(light_to_temerature) = parse_map(&mut lines) else {
        return Err("Failed to parse light_to_temerature");
    };

    let Some(temerature_to_humidity) = parse_map(&mut lines) else {
        return Err("Failed to parse temerature_to_humidity");
    };

    let Some(humidity_to_location) = parse_map(&mut lines) else {
        return Err("Failed to parse humidity_to_location");
    };
 
    Ok(Input {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temerature,
        temerature_to_humidity,
        humidity_to_location,
    })
}


fn parse_map(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<MapElement>> {
    let mut elements = Vec::new();

    while let Some(l) = lines.next() {
        let Ok(line) = l else {
            break;
        };

        match parse_map_element(&line) {
            Some(el) => elements.push(el),
            None => {
                if elements.len() == 0 {
                    continue; /* Skipping text at start */
                } else {
                    break; /* break in map */
                }
            }
        }
    }

    elements.sort();

    if elements.len() == 0 {
        None 
    } else {
        Some(elements)
    }
}


fn parse_map_element(line: &str) -> Option<MapElement> {
    let mut nums = Vec::new();
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().peekable();

    loop {
        let Some(n) = parse_num(&mut chars, true) else {
            break;
        };

        nums.push(n as u64);
    }

    return if nums.len() != 3 {
        None
    } else {
        Some(MapElement{
            des_range_start: nums[0],
            src_range_start: nums[1],
            length: nums[2]
        })
    };
}


fn parse_seeds(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<u64>>{
    let Ok(line) = lines.next()? else {
        return None;
    };
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().peekable();
    let mut seeds = Vec::new();

    loop {
        let Some(n) = parse_num(&mut chars, true) else {
            break;
        };

        seeds.push(n as u64);
    }

    Some(seeds)
}

fn print_input(input: &Input) {
    println!("Seeds:");
    print!("    ");

    for s in &input.seeds {
        print!("{} ", s);
    }

    println!("");

    println!("seed_to_soil: ");
    print_mapping(&input.seed_to_soil);
    println!("soil_to_fertilizer: ");
    print_mapping(&input.soil_to_fertilizer);
    println!("fertilizer_to_water: ");
    print_mapping(&input.fertilizer_to_water);
    println!("water_to_light: ");
    print_mapping(&input.water_to_light);
    println!("light_to_temerature: ");
    print_mapping(&input.light_to_temerature);
    println!("temerature_to_humidity: ");
    print_mapping(&input.temerature_to_humidity);
    println!("humidity_to_location: ");
    print_mapping(&input.humidity_to_location);
}

fn print_mapping(input: &Vec<MapElement>) {
    for el in input {
        println!("    s{} d{} l{}", 
            el.src_range_start, el.des_range_start, el.length
        );
    }
}