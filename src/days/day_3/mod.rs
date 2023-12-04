use crate::days::Part;

mod part1;
mod part2;

pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
    match part {
        Part::P1 => part1::run(file_name),
        Part::P2 => part2::run(file_name),
    }
}