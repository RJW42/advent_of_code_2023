use crate::days::Part;
use crate::days::{read_lines};

use self::Element::*;


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Element {
    Empty,
    Galxiy
}

pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
    match part {
        Part::P1 => part2(file_name),
        Part::P2 => todo!(),
    }
}



fn part1(file_name: &str) -> Result<u32, &'static str> {
    let mut map = parse_map(file_name)?;

    _print_galixy(&map);

    expand_galixy(&mut map);

    println!();
    _print_galixy(&map);

    let points = get_galixies(&map);
    let mut distances = 0;

    println!("{:?}", points);

    let mut count = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = &points[i];
            let (x2, y2) = &points[j];

            let distance = 
                (*x2 as i32 - *x1 as i32).abs() + (*y2 as i32 - *y1 as i32).abs()
            ;

            distances += distance as u32;
            count += 1;
            println!("{} {} {}", distance, i , j);
        }
    }

    println!("{}", count);

    Ok(distances)
}


fn part2(file_name: &str) -> Result<u32, &'static str> {
    let map = parse_map(file_name)?;

    _print_galixy(&map);

    let (empty_rows, empty_cols) = get_empty(&map);

    let points = get_galixies(&map);

    let mut distances = 0;
    let mut count = 0;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let (x1, y1) = &points[i];
            let (x2, y2) = &points[j];

            let mut distance = 
                (*x2 as i32 - *x1 as i32).abs() + (*y2 as i32 - *y1 as i32).abs()
            ;

            for empty_row in &empty_rows {
                if (*y1 < *y2 && *y1 < *empty_row && *empty_row < *y2) || 
                    (*y2 < *y1 && *y2 < *empty_row && *empty_row < *y1) {
                    distance += 1;
                }
            }


            for empty_col in &empty_cols {
                if (*x1 < *x2 && *x1 < *empty_col && *empty_col < *x2) || 
                    (*x2 < *x1 && *x2 < *empty_col && *empty_col < *x1) {
                    distance += 1;
                }
            }

            distances += distance as u32;
            count += 1;
            println!("{} {} {}", distance, i , j);
        }
    }

    println!("{}", count);

    Ok(distances)
}


fn get_galixies(map: &Vec<Vec<Element>>) -> Vec<(u32, u32)> {
    let mut output = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Empty {
                continue;
            }

            output.push(
                (x as u32, y as u32)
            );
        }
    }

    return output;
}


fn expand_galixy(galixy: &mut Vec<Vec<Element>>) {
    let mut width = galixy[0].len();
    let mut height = galixy.len();
    let mut row = 0;
    let mut column = 0;

    /* Add new Rows */
    loop {
        if row >= height {
            break;
        }

        let mut all_empty = true;

        for i in 0..width {
            if galixy[row][i] != Empty {
                all_empty = false;
                break;
            }
        }

        if !all_empty {
            row += 1;
            continue;
        }

        let new_row = vec![vec![Empty; width]];
        galixy.splice(row..row, new_row);

        row += 2;
        height += 1;
    }

    /* Add new columns */ 
    loop {
        if column >= width {
            break;
        }

        let mut all_empty = true;

        for i in 0..height {
            if galixy[i][column] != Empty {
                all_empty = false;
                break;
            }
        }

        if !all_empty {
            column += 1;
            continue;
        }

        for i in 0..height {
            let new_element = vec![Empty];
            galixy[i].splice(column..column, new_element);
        }

        column += 2;
        width += 1;
    }
}


fn get_empty(galixy: &Vec<Vec<Element>>) -> (Vec<u32>, Vec<u32>) {
    let width = galixy[0].len();
    let height = galixy.len();
    let mut row = 0;
    let mut column = 0;

    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    /* Add new Rows */
    loop {
        if row >= height {
            break;
        }

        let mut all_empty = true;

        for i in 0..width {
            if galixy[row][i] != Empty {
                all_empty = false;
                break;
            }
        }

        if all_empty {
            empty_rows.push(row as u32)
        }

        row += 1;
    }

    /* Add new columns */ 
    loop {
        if column >= width {
            break;
        }

        let mut all_empty = true;

        for i in 0..height {
            if galixy[i][column] != Empty {
                all_empty = false;
                break;
            }
        }

        if !all_empty {
            empty_cols.push(column as u32);
        }

        column += 1;
    }

    (empty_rows, empty_cols)
}


fn parse_map(file_name: &str) -> Result<Vec<Vec<Element>>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let mut output = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in file");
        };

        let mut row = Vec::new();

        for ch in line.chars() {
            match ch {
                '.' => row.push(Empty),
                '#' => row.push(Galxiy),
                _ => return Err("Invalid charicter found in input")
            };
        }

        output.push(row);
    }

    Ok(output)
}


fn _print_galixy(map: &Vec<Vec<Element>>) {

    for row in map {
        for el in row {
            match el {
                Empty => print!("."),
                Galxiy => print!("#"),
            };
        }

        println!();
    }
}