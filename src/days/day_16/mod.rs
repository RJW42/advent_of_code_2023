use std::usize;
use std::fmt;
use std::cmp::max;

use crate::days::Part;
use crate::days::{read_lines};

use self::Element::*;
use self::Direction::*;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Element {
    Empty,
    LeftMirror,
    RightMirror,
    VertialSpliter,
    HotizontalSpliter
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug)]
struct Input {
    elements: Vec<Element>,
    beams: Vec<Option<Vec<Direction>>>,
    width: usize,
    height: usize
}

type InputSize = usize;

fn part1(file_name: &str) -> Result<u64, &'static str> {
    let mut input = parse_input(file_name)?;

    Ok(calculate_covered(&mut input, 0, 0, East))
}


fn part2(file_name: &str) -> Result<u64, &'static str> {
    let mut input = parse_input(file_name)?;
    let mut max_covered = 0;

    for x in 0..input.width {
        let covered = calculate_covered(&mut input, x, 0, South);
        input.reset_beams();
        max_covered = max(covered, max_covered);

        let y = input.height - 1;
        let covered = calculate_covered(&mut input, x, y, North);
        input.reset_beams();
        max_covered = max(covered, max_covered);
    }

    for y in 0..input.height {
        let covered = calculate_covered(&mut input, 0, y, East);
        input.reset_beams();
        max_covered = max(covered, max_covered);

        let x = input.width - 1;
        let covered = calculate_covered(&mut input, x, y, West);
        input.reset_beams();
        max_covered = max(covered, max_covered);
    }


    Ok(max_covered)
}

fn calculate_covered(input: &mut Input, x: usize, y: usize, start_dir: Direction) -> u64 {
    let mut beam_heads = Vec::new();
    let start_pos = input.get_index(x, y).unwrap();

    for dir in input.new_beam_heads(start_pos, start_dir) {
        input.add_beam(start_dir, start_pos);
        beam_heads.push((x, y, dir));
    }

    loop {
        if beam_heads.len() == 0 {
            break;
        }

        let mut new_beam_heads = Vec::new();

        for (x, y, dir) in beam_heads {
            let (dx, dy) = match dir {
                North => (0, -1),
                South => (0, 1),
                East => (1, 0),
                West => (-1, 0)
            };

            let Some(new_pos) = input.try_get_index(x, y, dx, dy) else {
                continue;
            };
            let nx = x.checked_add_signed(dx).unwrap();
            let ny = y.checked_add_signed(dy).unwrap();
            let dirs = input.new_beam_heads(new_pos, dir);

            for dir in dirs {
                if !input.add_beam(dir, new_pos) {
                    continue;
                }

                new_beam_heads.push((nx, ny, dir));
            }
        }

        // println!("{}", input);
        beam_heads = new_beam_heads;
    }

    // println!("{}", input);

    input.beams.iter()
        .map(|s| if s.is_some() {1} else {0})
        .reduce(|acc, x| acc + x).unwrap()
}


impl Input {
    fn add_beam(&mut self, dir: Direction, i: InputSize) -> bool {
        if self.beams[i] == None {
            self.beams[i] = Some(vec![dir]);
            return true; 
        }

        let beams = self.beams.get_mut(i).unwrap().as_mut().unwrap();

        if beams.contains(&dir) {
            return false;
        }

        beams.push(dir);
        return true;
    }

    fn get_index(&self, x: usize, y: usize) -> Option<InputSize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width) + x)
        }
    }

    fn try_get_index(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<InputSize> {
        let nx = x.checked_add_signed(dx);
        let ny = y.checked_add_signed(dy);

        match (nx, ny) {
            (Some(nx), Some(ny)) => self.get_index(nx, ny),
            _ => None
        }
    }

    fn reset_beams(&mut self) {
        for i in 0..self.beams.len() {
            self.beams[i] = None;
        }
    }

    fn new_beam_heads(&self, pos: InputSize, dir: Direction) -> Vec<Direction> {
        match (&self.elements[pos], &dir) {
            (RightMirror, East) | 
            (LeftMirror, West) => {
                vec![North]
            },
            (RightMirror, West) |
            (LeftMirror, East) => {
                vec![South]
            },
            (RightMirror, North) | 
            (LeftMirror, South) => {
                vec![East]
            },
            (RightMirror, South) | 
            (LeftMirror, North) => {
                vec![West]
            },
            (HotizontalSpliter, South) | 
            (HotizontalSpliter, North) => {
                vec![East, West]
            },
            (VertialSpliter, East) |
            (VertialSpliter, West) => {
                vec![North, South]
            }
            (_, _) => {
                vec![dir]
            }
        }
    }

}


impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.get_index(x, y).unwrap();

                match &self.beams[i] {
                    Some(beams) if 
                    self.elements[i] == Empty => {
                        if beams.len() > 1 {
                            write!(f, "{}", beams.len())?;
                        } else {
                            let ch = match &beams[0] {
                                North => '^',
                                East => '>',
                                South => 'v',
                                West => '<',
                            };

                            write!(f, "{}", ch)?;
                        }
                    },
                    _ => write!(f, "{}", self.elements[i])?,
                }
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}



impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            LeftMirror => '\\',
            RightMirror => '/',
            HotizontalSpliter => '-',
            VertialSpliter => '|',
            Empty => '.'
        };

        write!(f, "{}", ch)
    }
}




fn parse_input(file_name: &str) -> Result<Input, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to open file");
    };

    let mut width = 0;
    let mut height = 0;
    let mut elements = Vec::new();
    let mut beams = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in file");
        };

        for c in line.chars() {
            let element = match c {
                '|' => VertialSpliter,
                '-' => HotizontalSpliter,
                '/' => RightMirror,
                '\\' => LeftMirror,
                '.' => Empty,
                _ => return Err("Invalid char found in input")
            };

            elements.push(element);
            beams.push(None);
        }

        if width == 0 {
            width = elements.len();
        }

        height += 1;
    }

    Ok(Input {
        elements,
        width,
        height,
        beams
    })
}