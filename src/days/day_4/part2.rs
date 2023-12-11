use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn run(file_name: &str) -> Result<u64, &'static str> {
    let mut output = 0;
    let mut cards = Vec::<(Vec<u64>, Vec<u64>)>::new();
    let mut card_amounts = Vec::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(line) = line {
                let mut c_ns = Vec::new();
                let mut w_ns = Vec::new();

                parse_line(&mut c_ns, &mut w_ns, &line);

                cards.push((c_ns, w_ns));
                card_amounts.push(1);
            }
        }
    }

    for (i, (c_ns, w_ns)) in cards.iter().enumerate() {
        let mut winnings = 0;
        
        for n in c_ns {
            if w_ns.contains(n) {
                winnings += 1;
                continue;
            }
        }

        // println!(" - {} {}", i, winnings);

        for j in (i + 1)..(i + winnings + 1) {
            if j >= card_amounts.len() {
                break;
            }
            card_amounts[j] += card_amounts[i];
            // println!("    - {} {}", j, card_amounts[j]);
        }
    }

    for n in card_amounts {
        output += n;
    }


    Ok(output)
}

fn parse_line(card_numbers: &mut Vec<u64>, winning_numbers: &mut Vec<u64>, line: &str) {
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
            current_number = current_number * 10 + (c as u64 - '0' as u64);
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

