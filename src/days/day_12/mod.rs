use crate::days::Part;
use crate::days::{read_lines, parse_num};

use self::Element::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Element {
    Empty,
    Gear,
    Unkown,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Input {
    elements: Vec<Element>,
    required_gears: Vec<u64>,
    amount_to_touch: usize
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}



fn part1(file_name: &str) -> Result<u64, &'static str> {
    let inputs = parse_inputs(file_name)?;
    let mut output = 0;

    for input in inputs {
        let ways = count_ways(&input);

        println!("  - {}", ways);

        output += ways;
    }


   Ok(output)
}


fn part2(file_name: &str) -> Result<u64, &'static str> {
    let inputs = parse_inputs(file_name)?;
    let inputs = unwrap_inputs(&inputs);
    let mut output = 0;

    for input in inputs {
        let ways = count_ways(&input);

        println!("  - {}", ways);

        output += ways;
    }


   Ok(output)
}

fn count_ways(input: &Input) -> u64 {
    let mut dp = Vec::new();

    for _ in 0..input.required_gears.len() {
        let mut r = Vec::new();
        
        for _ in 0..input.elements.len() {
            r.push(vec![None; input.amount_to_touch + 1]);
        }

        dp.push(r);
    }

    // Too high 7402
    solve(&mut dp, input, 0, 0, 0) as u64
}

fn solve(
        dp: &mut Vec<Vec<Vec<Option<u64>>>>, input: &Input, gear: usize, 
        start_pos: usize, amount_touched: usize
    ) -> u64 {
    // dp[gear][start_pos] = number of ways to place gear starting at start_pos 
    if dp[gear][start_pos][amount_touched] != None {
        return dp[gear][start_pos][amount_touched].unwrap();        
    }

    let mut ways = 0;

    for i in start_pos..input.elements.len() {
        let mut current_touched = 0;

        let placable = can_place(
            input, i, input.required_gears[gear], &mut current_touched
        );

        if !placable {
            continue;
        }
        
        if gear == input.required_gears.len() - 1 {
            if amount_touched + current_touched != input.amount_to_touch {
                continue; // not covered all existing gears 
            }
            ways += 1; // base case final gear
            continue;
        }

        let end_pos = i + input.required_gears[gear] as usize + 1;

        if end_pos >= input.elements.len() {
            break; // base case not enough space for remaning gears, so cannot place 
        } 

        // dp, is enough space, can place this gear, check all other gearss
        if dp[gear + 1][end_pos][amount_touched + current_touched] != None {
            ways += dp[gear + 1][end_pos][amount_touched + current_touched].unwrap();
            continue;
        }

        // not visited next gear 
        let amount = solve(
            dp, input, gear + 1, end_pos,
            amount_touched + current_touched
        );

        ways += amount;
    }

    dp[gear][start_pos][amount_touched] = Some(ways);

    return ways;
}

fn can_place(
        input: &Input, start_pos: usize, gear_size: u64, current_touched: &mut usize
) -> bool {
    if start_pos + gear_size as usize > input.elements.len() {
        return false;
    }

    if start_pos != 0 && input.elements[start_pos - 1] == Gear {
        return false; // No space before
    }

    for i in start_pos..(start_pos + gear_size as usize) {
        match input.elements[i] {
            Empty => return false,
            Gear => *current_touched += 1,
            Unkown => (),
        };
    }

    if start_pos + gear_size as usize == input.elements.len() {
        return true; // Enough space afterwards
    }

    match input.elements[start_pos + gear_size as usize] {
        Empty => true,
        Unkown => true,
        Gear => false,
    }
}


fn parse_inputs(file_name: &str) -> Result<Vec<Input>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read input file");
    };

    let mut output = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in input file");
        };

        let mut elements = Vec::new();
        let mut required_gears = Vec::new();
        let chars_vec = line.chars().collect::<Vec<char>>();
        let mut chars = chars_vec.iter().peekable();
        let mut amount_to_touch = 0;

        loop {
            let element = match chars.next() {
                Some('.') => Empty,
                Some('?') => Unkown,
                Some('#') => {
                    amount_to_touch += 1;
                    Gear
                },
                Some(' ') => break,
                _ => return Err("Invalid char found in input")
            };

            elements.push(element);
        }

        loop {
            let Some(n) = parse_num(&mut chars, true) else {
                break;
            };

            required_gears.push(n as u64);
        }

        output.push( Input {
            elements, required_gears, amount_to_touch
        });
    }

    Ok(output)
}


fn unwrap_inputs(inputs: &Vec<Input>) -> Vec<Input> {
    let mut output = Vec::new();
    let multiuply_amount = 5;

    for input in inputs {
        let amount_to_touch = input.amount_to_touch * multiuply_amount;
        let mut elements = Vec::new();
        let mut required_gears = Vec::new();

        for j in 0..multiuply_amount{
            for i in 0..input.elements.len() {
                elements.push(input.elements[i]);   
            }
            
            if j != multiuply_amount - 1 {
                elements.push(Unkown);
            }

            for i in 0..input.required_gears.len() {
                required_gears.push(input.required_gears[i]);   
            }
        }

        output.push( Input {
            required_gears, elements, amount_to_touch
        });
    }

    output
}