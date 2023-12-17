use crate::days::Part;
use crate::days::{read_lines};

use std::collections::{HashMap, BinaryHeap};
use std::slice::Iter;
use std::fmt;

use self::Direction::*;

#[derive(Debug)]
struct Input {
    nodes: Vec<Node>,
    width: usize,
    height: usize
}

type InputSize = usize;

#[derive(Debug)]
struct Node {
    heat_loss: u8
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    Void
}


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}

fn part1(file_name: &str) -> Result<u64, &'static str> {
    let input = parse_input(file_name)?;

    println!("{}", input);
    
    Ok(dijkstra(&input, 1, 3))
}

fn part2(file_name: &str) -> Result<u64, &'static str> {
    let input = parse_input(file_name)?;

    // println!("{}", input);
    
    Ok(dijkstra(&input, 4, 10))
}


fn dijkstra(input: &Input, min_step: usize, max_step: usize) -> u64 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start_pos = (0, 0);
    let end_pos = (input.width, input.height);

    queue.push((0, (start_pos.0, start_pos.1, Void)));

    while let Some((cost, (x, y, dir))) = queue.pop() {
        // println!("{} {} {:?}", x, y, dir);
    
        if x == end_pos.0 - 1 && y == end_pos.1 - 1 {
            return (-cost) as u64;
        }

        if dists.get(&(x, y, dir)).is_some_and(|&c| -cost > c) {
            continue;
        }

        for n_dir in Direction::iterator() {
            if &dir == n_dir || &dir.invert() == n_dir{
                continue;
            }

            let mut n_cost = -cost; // Need to store cost negated in queue, so invert to get true cost 

            for dist in 1..=max_step {
                let Some((node, n_x, n_y)) = input.try_get_move(x, y, *n_dir, dist) else {
                    break;
                };

                n_cost += node.heat_loss as i64;

                if dist < min_step {
                    continue;
                }

                let key = (n_x, n_y, *n_dir);

                if n_cost < *dists.get(&key).unwrap_or(&i64::MAX) {
                    dists.insert(key, n_cost);
                    queue.push((-n_cost, key));
                }
            }
        }
    }

    unreachable!()
}


impl Direction {
    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, South, East, West];

        DIRECTIONS.iter()
    }

    fn invert(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            Void => Void
        }
    }
}


impl Input {
    fn get<'a>(&'a self, i: InputSize) -> &'a Node {
        &self.nodes[i]
    }

    fn try_get<'a>(&'a self, x: usize, y: usize) -> Option<&'a Node> {
       let i = self.try_get_index(x, y)?;
       Some(self.get(i))
    }

    fn try_get_move<'a>(&'a self, x: usize, y: usize, dir: Direction, dist: usize) -> Option<(&'a Node, usize, usize)> {
        let (dx, dy) = match dir {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
            Void => panic!(),
        };

        let dx = dx * dist as isize;
        let dy = dy * dist as isize;

        let (nx, ny) = self.try_get_change(x, y, dx, dy)?; 
        let i = self.try_get_index(nx, ny)?;

        Some((self.get(i), nx, ny))
    }

    fn try_get_index(&self, x: usize, y: usize) -> Option<InputSize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width) + x)
        }
    }

    fn try_get_change(&self, x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
        let nx = x.checked_add_signed(dx)?;
        let ny = y.checked_add_signed(dy)?;

        self.try_get_index(nx, ny)?;

        Some((nx, ny))
    }

}


impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.try_get(x, y).unwrap().heat_loss)?;
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}


fn parse_input(file_name: &str) -> Result<Input, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read file")
    };

    let mut nodes = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for l in lines {
        let Ok(line) = l else {
            return Err("Failed to read line in input")
        };

        for ch in line.chars() {
            if !('0'..='9').contains(&ch) {
                return Err("Invalid char found in input");
            }

            let num = ch as u8 - '0' as u8;

            nodes.push(Node {
                heat_loss: num
            });
        }

        if width == 0 {
            width = nodes.len();
        }

        height += 1;
    }

    Ok(Input {
        nodes,
        width,
        height
    })
}