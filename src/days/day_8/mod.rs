use crate::days::Part;
use crate::days::{read_lines};

use std::fs::File;
use std::io::{self};
use std::iter::Peekable;
use num::integer::lcm;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Left,
    Right
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Node {
    id: u32,
    left_id: u32,
    right_id: u32,
    is_end: bool
}

static START_ID: u32 = 0;
static END_ID: u32 = 17575;


pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
  }
  

fn part1(file_name: &str) -> Result<u32, &'static str> {
    let (path, graph, _) = parse_input(file_name)?;
    
    let mut i = 0;
    let mut amount_walked = 0;

    let mut current_node = graph[START_ID as usize].unwrap();


    loop {
        let Some(node) = (match &path[i % path.len()] {
            Action::Left => &graph[current_node.left_id as usize],
            Action::Right => &graph[current_node.right_id as usize]
        }) else {
            println!("Failed to find node from {}", current_node.id);
            break;
        };

        amount_walked += 1;
        i += 1;
        println!("{:?}", node);


        if node.id == END_ID {
            break;
        }

        current_node = *node;
    }
  
    Ok(amount_walked)
}


fn part2(file_name: &str) -> Result<u32, &'static str> {
    let (path, graph, start_nodes) = parse_input(file_name)?;

    println!("{:?}", path);
    
    let mut current_nodes = Vec::new();

    for id in start_nodes {
        let Some(_node) = graph[id as usize] else {
            return Err("Failed to find start node");
        };
        current_nodes.push((id as usize, 0 as u32));
    }

    println!("{:?}", current_nodes);

    let current_nodes_len = current_nodes.len();


    for cn_i in 0..current_nodes_len {
        let mut i = 0;
        let mut amount_walked = 0;

        loop {
            let action = &path[i % path.len()];
        
            let (cn_id, _cn_s) = current_nodes[cn_i];

            let node_id = match action {
                Action::Left => graph[cn_id].unwrap().left_id as usize,
                Action::Right => graph[cn_id].unwrap().right_id as usize
            };

            i += 1;
            amount_walked += 1;

            current_nodes[cn_i] = (node_id, amount_walked);

            if graph[node_id].unwrap().is_end {
                break;
            }
        }

        println!("node: {:?}, {}", graph[current_nodes[cn_i].0], current_nodes[cn_i].1);
    }    

    let mut result = 1;

    for (_, l) in &current_nodes {
        println!("{}", l);
        result = lcm(result, *l as u64);
    }

  
    Ok(result as u32)
}


fn parse_input(file_name: &str) -> Result<(Vec<Action>, [Option<Node>; 17576], Vec<u32>), &'static str> {
    let Ok(mut lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let Some(path) = parse_path(&mut lines) else {
        return Err("Failed to parse path");
    };


    lines.next(); /* Skip emptyy line */ 

    let Some((graph, start_nodes)) = parse_graph(&mut lines) else {
        return Err("Failed to parse graph");
    };


    Ok((path, graph, start_nodes))
}



fn parse_path(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<Action>> {
    let Ok(line) = lines.next()? else {
        return None;
    };
    let mut actions = Vec::new();

    for c in line.chars() {
        match c {
            'L' => actions.push(Action::Left),
            'R' => actions.push(Action::Right),
            _ => break,
        };
    }


    Some(actions)
}


fn parse_graph(lines: &mut io::Lines<io::BufReader<File>>) -> Option<([Option<Node>; 17576], Vec<u32>)> {
    let mut start_nodes = Vec::new();
    let mut graph: [Option<Node>; 17576] = [None; 17576];

    for l in lines {
        let Ok(line) = l else {
            return None;
        };

        let Some(n) = parse_node(&line, &mut start_nodes) else {
            return None;
        };

        println!("{:?}", n);
        graph[n.id as usize] = Some(n);
    }

    Some((graph, start_nodes))
}


fn parse_node(line: &str, start_nodes: &mut Vec<u32>) -> Option<Node> {
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().peekable();

    let (id, is_start, is_end) = parse_id(&mut chars)?;
    let (left_id, _, _) = parse_id(&mut chars)?;
    let (right_id, _, _) = parse_id(&mut chars)?;

    if is_start {
        start_nodes.push(id);
    }

    Some(
        Node {
            id,
            left_id,
            right_id,
            is_end
        }
    )
}


fn parse_id<'a, I>(chars: &mut Peekable<I>) -> Option<(u32, bool, bool)> 
where I: Iterator<Item = &'a char>  {
    let mut output = None;
    let mut output_val = 0;
    let mut length = 0;
    let mut is_end = false;
    let mut is_start = false;


    loop {
        match chars.peek() {
            ch @ Some('A'..='Z') => {
                let ch = **ch.unwrap();
                length += 1;

                if length == 3 && ch == 'A' {
                    is_start = true;
                } else if length == 3 && ch =='Z' {
                    is_end = true;
                }

                output_val = output_val * 26 + (
                    ch as u32 - 'A' as u32
                );

                output = Some(output_val);
                chars.next();
            }
            None => break,
            _ => {
                if output == None {
                    chars.next();
                } else {
                    break;
                }
            }
        }
    }

    match output {
        Some(n) => Some((n, is_start, is_end)),
        None => None
    }
}