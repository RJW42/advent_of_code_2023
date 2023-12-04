use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;


fn main() {
    let mut output = 0;

    if let Ok(lines) = read_lines("./inputs/day_4.txt") {
        for line in lines {
            if let Ok(line) = line {
                let mut c_ns = Vec::new();
                let mut w_ns = Vec::new();
                let mut card_score = 0;

                parse_line(&mut c_ns, &mut w_ns, &line);

                for n in &c_ns {
                    if w_ns.contains(n) {
                        if card_score == 0 {
                            card_score = 1;
                        } else {
                            card_score *= 2;
                        }
                        continue;
                    }
                }
                
                output += card_score;
                // println!("{} {} {}", c_ns.len(), w_ns.len(), output);
            }
        }
    }


    println!("{}", output);
}

fn parse_line(card_numbers: &mut Vec<u32>, winning_numbers: &mut Vec<u32>, line: &str) {
    let mut start = false;
    let mut parsed_cn = false;

    let mut current_number = 0;

    for c in line.chars() {
        if !start {
            if c == ':' {
                start = true;
            }
            continue;
        }

        if c >= '0' && c <= '9' {
            current_number = current_number * 10 + (c as u32 - '0' as u32);
            continue;
        }

        if c == '|' {
            parsed_cn = true;
            continue;
        }

        if current_number == 0 {
            continue;
        }

        if parsed_cn {
            winning_numbers.push(current_number);
        } else {
            card_numbers.push(current_number);
        }
        current_number = 0;
    }
    winning_numbers.push(current_number);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

