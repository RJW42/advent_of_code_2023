use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


enum Entry {
    Symbol(char),
    Number(u32, bool),
    Empty
}


pub fn run(file_name: &str) -> Result<u32, &'static str> {
    let mut grid: Vec<Entry> = Vec::new();
    let mut symbol_poss: Vec<(usize, usize)> = Vec::new();
    let mut width = 0;
    let mut height = 0;

    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            if let Ok(line) = line {
                parse_line(&mut grid, &mut symbol_poss, &line, height);
                height += 1 as usize;

                if width == 0 {
                    width = grid.len();
                }
            }
        }
    }


    for (s_x, s_y) in symbol_poss {
        let x = s_x as i32;
        let y = s_y as i32;
        // TL
        if is_valid(x - 1, y -1, width, height) {
            mark_visited(x - 1, y - 1, width, &mut grid);
        }
        // T
        if is_valid(x, y -1, width, height) {
            mark_visited(x, y - 1, width, &mut grid);
        }
        // TR
        if is_valid(x + 1, y - 1, width, height) {
            mark_visited(x + 1, y - 1, width, &mut grid);
        }
        // L
        if is_valid(x - 1, y, width, height) {
            mark_visited(x - 1, y, width, &mut grid);
        }
        // R
        if is_valid(x + 1, y, width, height) {
            mark_visited(x + 1, y, width, &mut grid);
        }
        // BL
        if is_valid(x - 1, y + 1, width, height) {
            mark_visited(x - 1, y + 1, width, &mut grid);
        }
        // B 
        if is_valid(x, y + 1, width, height) {
            mark_visited(x, y + 1, width, &mut grid);
        }
        // BR 
        if is_valid(x + 1, y + 1, width, height) {
            mark_visited(x + 1, y + 1, width, &mut grid);
        }
    }

    // print_grid(&grid, width, height);

    let mut sum = 0;

    for y in 0..height {
        let mut curr_number_visited = false;
        for x in 0..width {
            let pos = (y * width) + x;
            match grid[pos as usize] {
                Entry::Empty => (), Entry::Symbol(_) => (),
                Entry::Number(n, v) => {
                    curr_number_visited = curr_number_visited || v;

                    if (x + 1) < width {
                        if let Entry::Number(_, v2) = grid[pos + 1] {
                            curr_number_visited = curr_number_visited || v2;
                            continue;
                        } 
                    }

                    if curr_number_visited {
                        sum += n;
                    } 
                    curr_number_visited = false;
                },
            }
        }
    }

    Ok(sum)
}

fn mark_visited(x: i32, y: i32, width: usize, grid: &mut Vec<Entry>) {
    let pos = (y * width as i32) + x;
    if let Entry::Number(n, _) = grid[pos as usize] {
        grid[pos as usize] = Entry::Number(n, true);
    }
}

fn is_valid(x: i32, y: i32, width: usize, height: usize) -> bool {
    return x >= 0 && y >= 0 && x < width as i32 && y < height as i32;
}


fn parse_line(grid: &mut Vec<Entry>, symbol_poss: &mut Vec<(usize, usize)>,line: &str, height: usize) {
    let mut current_number = 0;
    let mut current_number_size = 0;

    for (i, c) in line.chars().enumerate() {
        if (c < '0' || c > '9') && current_number != 0 {
            for _i in 0..current_number_size {
                grid.push(Entry::Number(current_number, false));
            }
            current_number = 0;
            current_number_size = 0;
        } 
        
        if c >= '0' && c <= '9' {
            current_number_size += 1;
            current_number = current_number * 10 + (c as u32 - '0' as u32);
        } else if c == '.' {
            grid.push(Entry::Empty);
        } else {
            symbol_poss.push((i, height));
            grid.push(Entry::Symbol(c));
        }
    }   

    if current_number != 0 {
        for _ in 0..current_number_size {
            grid.push(Entry::Number(current_number, false));
        }
    } 
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn _print_grid(grid: &Vec<Entry>, width: usize, height: usize) {
    for y in 0..height {
        let mut curr_number_visited = false;
        for x in 0..width {
            let pos = (y * width) + x;
            match grid[pos as usize] {
                Entry::Empty => print!("."),
                Entry::Symbol(c) => print!("{}", c),
                Entry::Number(n, v) => {
                    curr_number_visited = curr_number_visited || v;

                    if (x + 1) < width {
                        if let Entry::Number(_, v2) = grid[pos + 1] {
                            curr_number_visited = curr_number_visited || v2;
                            continue;
                        } 
                    }

                    if curr_number_visited {
                        print!("\x1b[92m{}\x1b[0m", n);
                    } else {
                        print!("\x1b[91m{}\x1b[0m", n)
                    }
                    curr_number_visited = false;
                },
            }
        }
        println!();
    }
}
