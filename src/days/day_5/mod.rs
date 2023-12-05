use crate::days::Part;
use crate::days::{read_lines, parse_num};

use std::fs::File;
use std::io::{self};
use std::cmp::Ordering;

struct Input {
    seeds: Vec<u32>,
    soil_to_soil: Vec<MapElement>,
    soil_to_fertilizer: Vec<MapElement>,
    fertilizer_to_water: Vec<MapElement>,
    water_to_light: Vec<MapElement>,
    light_to_temerature: Vec<MapElement>,
    temerature_to_humidity: Vec<MapElement>,
    humidity_to_location: Vec<MapElement>
}

#[derive(Eq)]
struct MapElement {
    des_range_start: u32,
    src_range_start: u32,
    length: u32
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

    Ok(0)
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

    let Some(soil_to_soil) = parse_map(&mut lines) else {
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
        soil_to_soil,
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

    println!("a");

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

        nums.push(n as u32);
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


fn parse_seeds(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<u32>>{
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

        seeds.push(n as u32);
    }

    Some(seeds)
}

fn print_input(input: &Input) {
    println!("Seeds:");

    for s in &input.seeds {
        print!("{} ", s);
    }

    println!("")
}