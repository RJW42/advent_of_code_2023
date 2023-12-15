pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::iter::Peekable;

pub enum Part {
    P1,
    P2
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn parse_num<'a, I>(chars: &mut Peekable<I>, skip_non_numeric: bool) -> Option<i64> 
where I: Iterator<Item = &'a char> {
    let mut output = None;
    let mut output_val = 0;
    let mut sign = 1;

    loop {
        match chars.peek() {
            Some('-') => {
                sign = -1;
                chars.next();
            },
            ch @ Some('0'..='9') => {
                output_val = output_val * 10 + (
                    **ch.unwrap() as i64 - '0' as i64
                );
                output = Some(output_val);
                chars.next();
            },
            None => break,
            _ => {
                if skip_non_numeric && output == None {
                    chars.next();
                    continue
                } else {
                    break;
                }
            },
        };
    }

    output.map(|v| v * sign)
}