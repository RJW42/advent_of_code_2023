use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::days::Part;


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    let mut product = 0;
    let use_words = match part {
        Part::P1 => false,
        Part::P2 => true
    };

    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read lines");
    };

    
    for line in lines {
        if let Ok(line) = line {
            let v = parse_line(&line, use_words);
            
            let n = if v.len() == 0 {
                0
            } else {
                v[0] * 10 + v[v.len() - 1]
            };

            
            product += n;
        }
    }

    Ok(product)
}

fn parse_line(line: &str, use_words: bool) -> Vec<u64> {
    let mut digits = Vec::new();
    let mut prev_digit_index = 0 as usize;

    for (i, c) in line.chars().enumerate() {
        let maybe_digit = parse_digit(c);

        if let Some(digit) = maybe_digit {
            digits.push(digit);
            prev_digit_index = i + 1;
            continue;
        }
        
        if !use_words {
            continue;
        }


        for j in prev_digit_index..i {
            let slice = &line[j..(i + 1)];
            let len = digits.len();

            // println!("  - {} ", slice);

            match slice {
                "one" => digits.push(1),
                "two" => digits.push(2),
                "three" => digits.push(3),
                "four" => digits.push(4),
                "five" => digits.push(5),
                "six" => digits.push(6),
                "seven" => digits.push(7),
                "eight" => digits.push(8),
                "nine" => digits.push(9),
                _ => ()
            };

            if len != digits.len() {
                prev_digit_index = i;
                break;
            }
        }
    }

    return digits;
}

fn parse_digit(c: char) -> Option<u64> {
    let d = c as i32 - '0' as i32;

    if !(d >= 0 && d <= 9)  {
        return None;
    }

    Some(d as u64)
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}