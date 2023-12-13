use crate::days::Part;
use crate::days::{read_lines};

use self::Element::*;
use self::MirrorPoint::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Element {
    Empty,
    Rock
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum MirrorPoint {
    Row,
    Col
}

#[derive(Debug)]
struct Input {
    rows: Vec<u64>,
    cols: Vec<u64>
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name, true),
        Part::P2 => todo!(),
    }
}


fn part1(file_name: &str, use_smudge: bool) -> Result<u64, &'static str> {
    let inputs = parse_input(file_name)?;
    let mut score = 0;

    for input in &inputs {
        let mirrow_point = find_mirror_points(input, use_smudge);
        // Too high 37240
        //          36015

        let value = match mirrow_point {
            Some((Row, n)) => n as u64 * 100,
            Some((Col, n)) => n as u64,
            None => panic!()
        };

        score += value;

        println!("{}", value);
    }   

    Ok(score)
}

fn find_mirror_points(input: &Input, use_smudge: bool) -> Option<(MirrorPoint, usize)> {
    let row_point = find_mirror_point(&input.rows, use_smudge);
    let col_point = find_mirror_point(&input.cols, use_smudge);

    match (row_point, col_point) {
        (None, None) => None,
        (None, Some(n)) => Some((Col, n + 1)),
        (Some(n), None) => Some((Row, n + 1)),
        (Some(a), Some(b)) => if a > b {
            Some((Row, a + 1))
        } else {
            Some((Col, b + 1))
        }
    }
}


fn find_mirror_point(list: &Vec<u64>, use_smudge: bool) -> Option<usize> {
    let mut point = 0;
    let mut point_size = 0;

    for i in 0..(list.len() - 1) {
        let mut is_perfect = true;
        let mut used_smudge = false;

        for size in 1..list.len() {
            let Some(left_i) = i.checked_sub(size - 1) else {
                break;
            };
            let right_i = i + size;

            if right_i >= list.len() {
                break;
            }

            if list[left_i] != list[right_i] && !use_smudge {
                is_perfect = false;
                break;
            }

            if list[left_i] != list[right_i] && use_smudge {
                let can_use_smudge = power_of_two(list[left_i] ^ list[right_i]);

                if used_smudge || !can_use_smudge {
                    is_perfect = false;
                    break;
                } else {
                    used_smudge = true;
                    continue;
                }
            }
        }

        if is_perfect && !use_smudge {
            point = i;
            point_size = 1;
        } else if used_smudge && is_perfect {
            point = i;
            point_size = 1;
        }
    }

    if point_size == 0 {
        None
    } else {
        Some(point)
    }
}


fn power_of_two(x: u64) -> bool {
    (x & (x - 1)) == 0
}


fn parse_input(file_name: &str) -> Result<Vec<Input>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read input file");
    };

    let mut elements = Vec::new();
    let mut inputs = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in input file");
        };

        if line.len() == 0 {
            inputs.push(input_from_elements(&elements));
            elements.clear();
            continue;
        }

        let mut row = Vec::new();

        for ch in line.chars() {
            match ch {
                '.' => row.push(Empty),
                '#' => row.push(Rock),
                _ => return Err("Invalid input char")
            };
        }

        elements.push(row)
    }

    inputs.push(input_from_elements(&elements));

    Ok(inputs)
}


fn input_from_elements(elements: &Vec<Vec<Element>>) -> Input {
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    for r in 0..elements.len() {
        let mut id = 0;

        for i in 0..elements[0].len() {
            id = (id << 1) + match elements[r][i] {
                Empty => 0,
                Rock => 1
            };
        }

        rows.push(id);
    }


    for c in 0..elements[0].len() {
        let mut id = 0;

        for i in 0..elements.len() {
            id = (id << 1) + match elements[i][c] {
                Empty => 0,
                Rock => 1
            };
        }

        cols.push(id);
    }

    Input {
        rows,
        cols
    }
}