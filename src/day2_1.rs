use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp;


struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32
}


fn _main() {
    let mut output = 0;

    if let Ok(lines) = read_lines("./inputs/day_2.txt") {
        for line in lines {
            if let Ok(line) = line {
                if let Some(game) = parse_game(&line) {
                    if !is_possible(&game) {
                        continue;
                    }

                    output += game.id;
                }
            }
        }
    }

    println!("{}", output);
}


fn parse_game(line: &str) -> Option<Game> {
    let mut last_number = 0;
    let mut game: Game = Game {
        id: 0,
        blue: 0,
        green: 0,
        red: 0
    };

    for c in line.chars() {
        if c == ' ' {
            continue;
        }

        if let Some(d) = parse_digit(c) {
            last_number = last_number * 10 + d;
            continue;
        }

        if last_number == 0 {
            continue;
        }

        match c {
            ':' => game.id = last_number,
            'b' => game.blue = cmp::max(game.blue, last_number),
            'r' => game.red = cmp::max(game.red, last_number),
            'g' => game.green = cmp::max(game.green, last_number),
            _ => continue,
        };

        last_number = 0;
    }

    Some(game)
}


fn parse_digit(c: char) -> Option<u32> {
    let d = c as i32 - '0' as i32;

    if !(d >= 0 && d <= 9)  {
        return None;
    }

    Some(d as u32)
}

fn is_possible(game: &Game) -> bool {
    return game.red <= 12 && game.green <= 13 && game.blue <= 14;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}