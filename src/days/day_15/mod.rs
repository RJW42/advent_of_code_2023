use std::mem::{MaybeUninit, transmute};
use std::usize;

use crate::days::Part;
use crate::days::{read_lines};

use self::Operation::*;

pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Operation {
    Insert,
    Delete
}

#[derive(Debug)]
struct Command<'a> {
    opp: Operation,
    id: &'a str,
    val: u32
}


fn part1(file_name: &str) -> Result<u64, &'static str> {
    let Ok(mut lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let Some(Ok(line)) = lines.next() else {
        return Err("Failed to read line");
    };

    let score = line
        .split(',').fold(0, |acc, s| acc + hash(s) as u64);

    Ok(score)
}


fn part2(file_name: &str) -> Result<u64, &'static str> {
    const LENGTH: usize = 256;

    let Ok(mut lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let Some(Ok(line)) = lines.next() else {
        return Err("Failed to read line");
    };

    let mut maps: [Vec<Command>; LENGTH] = {
        let mut data: [MaybeUninit<Vec<Command>>; LENGTH] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for elem in &mut data[..] {
            *elem = MaybeUninit::new(Vec::new());
        }

        unsafe { transmute::<_, [Vec<Command>; LENGTH]>(data)}
    };

    for step in line.split(',') {
        let Some(command) = get_command(step) else {
            return Err("Failed to aprse command");
        };

        let hash = hash(command.id) as usize;

        match command.opp {
            Insert => { 
                if maps[hash].contains(&command) {
                    let pos = maps[hash].iter().position(|c| c == &command).unwrap();
                    maps[hash][pos].val = command.val;
                } else {
                    maps[hash].push(command)
                }
            },
            Delete => maps[hash].retain(|c| c.id != command.id)
        };
    }

    let score = (0..LENGTH)
        .map(|i| maps[i]
            .iter().enumerate()
            .map(|(j, c)| (i + 1) as u64 * (j + 1) as u64 * c.val as u64)
            .fold(0, |acc, s| acc + s)
        ).fold(0, |acc, s| acc + s);

    Ok(score)
}


impl PartialEq for Command<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
    }
}


impl Eq for Command<'_> {

}


fn get_command<'a>(input: &'a str) -> Option<Command<'a>> {
    let mut val = 0;
    let mut opp = None;
    let mut str_end = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            'a'..='z' => str_end = i,
            n @ '0'..='9' => val = val * 10 + (n as u32 - '0' as u32),
            '=' => opp = Some(Insert),
            '-' => opp = Some(Delete),
            _ => return None
        }
    }

    if opp == None {
        None
    } else {
        Some(Command {
            opp: opp.unwrap(),
            id: &input[..(str_end + 1)],
            val,
        })
    }
}



fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, x| ((acc as u32 + x as u32) * 17) % 256)
}