use crate::days::Part;
use crate::days::{read_lines};
use std::fmt;

use self::Element::*;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Element {
    Empty,
    Rock,
    Wall
}

#[derive(Debug)]
struct Input {
    elements: Vec<Element>,
    width: usize,
    height: usize
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => todo!(),
    }
}


fn part1(file_name: &str) -> Result<u64, &'static str> {
    let mut input= parse_input(file_name)?;

    println!("{}", input);

    input.slide_north();

    println!("{}", input);

    Ok(count_score(&input))
}


fn count_score(input: &Input) -> u64 {
    let mut score = 0;


    for y in 0..input.height {
        let height_score = (input.height - y) as u64;

        for x in 0..input.width {
            if input.get(x, y) != Some(&Rock) {
                continue;
            }

            score += height_score;
        }
    }


    score
}


impl Input {
    fn get(&self, x: usize, y: usize) -> Option<&Element> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.elements[(y * self.width) + x])
    }

    fn set(&mut self, x: usize, y: usize, element: Element) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.elements[(y * self.width) + x] = element;
    }


    fn slide_north(&mut self) {
        let mut head = Vec::new(); // Stores top searching postion 
        let mut tail = Vec::new(); // Stores place to put element 

        if self.height == 1 {
            return;
        }

        for _x in 0..self.width {
            head.push(1);
            tail.push(0);
        }

        loop {
            let mut all_ended = true;
            // print!("{}\n\n", self);
            for (x, y) in head.iter().enumerate() {
                let y = *y;
                let Some(element) = self.get(x, y) else {
                    continue;
                };

                all_ended = false;

                match element {
                    Wall => {
                        tail[x] = y + 1;
                        continue;
                    }, // New tail
                    Empty => continue, // Do nothing 
                    Rock => (), // Need to push rock down
                };


                for new_y in tail[x]..(y + 1) {
                    match self.get(x, new_y).unwrap() {
                        Empty => {
                            // Place rock here 
                            self.set(x, y, Empty);
                            self.set(x, new_y, Rock);
                            tail[x] = new_y;
                            break;
                        },
                        Rock | Wall => {
                            tail[x] = new_y;
                        }
                    }
                }
            }


            if all_ended {
                break;
            }

            for x in 0..head.len() {
                head[x] += 1;
            }
        }
    }
}


fn parse_input(file_name: &str) -> Result<Input, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read input file");
    };

    let mut elements = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in input file");
        };

        for ch in line.chars() {
            match ch {
                '.' => elements.push(Empty),
                '#' => elements.push(Wall),
                'O' => elements.push(Rock),
                _ => return Err("Invalid input char")
            };
        }

        if width == 0 {
            width = elements.len();
        }
        height += 1;
    }

    Ok(Input {
        elements,
        width,
        height
    })
}


impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}



impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rock => write!(f, "O"),
            Wall => write!(f, "#"),
            Empty => write!(f, ".")
        }
    }
}