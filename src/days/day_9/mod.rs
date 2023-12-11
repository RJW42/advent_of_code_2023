use crate::days::Part;
use crate::days::{read_lines, parse_num};


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
    match part {
        Part::P1 => part1(file_name),
        Part::P2 => part2(file_name),
    }
}


fn part1(file_name: &str) -> Result<u64, &'static str> {
    let histories = parse_histories(file_name)?;
    let mut output = 0;

    for mut history in histories {
        // println!("{:?}", history);

        for head in 1..history.len() {
            // println!(" - {}", head);
            for curr in (1..(head + 1)).rev() {
                let prev = curr - 1;
                let diff = history[curr] - history[prev];

                history[prev] = diff;

                // println!("   - c{}, p{}, d{}", curr, prev, diff);
                // println!("   - {:?}", history);
            }
        }

        // println!("{:?}", history);

        let mut local_value = 0;

        for i in (0..history.len()).rev() {
            local_value += history[i];
        }

        // println!("{}", local_value);
        output += local_value;
    }

    // println!("{}", output);

    Ok(output as u64)
}


fn part2(file_name: &str) -> Result<u64, &'static str> {
    let histories = parse_histories(file_name)?;
    let mut output = 0;

    for history in histories {
        let mut history: Vec<i64> = history.into_iter().rev().collect();
        println!("{:?}", history);

        for head in 1..history.len() {
            println!(" - {}", head);
            for curr in (1..(head + 1)).rev() {
                let prev = curr - 1;
                let diff = history[curr] - history[prev];

                history[prev] = diff;

                println!("   - c{}, p{}, d{}", curr, prev, diff);
                println!("   - {:?}", history);
            }
        }

        println!("{:?}", history);

        let mut local_value = 0;

        for i in (0..history.len()).rev() {
            local_value += history[i];
        }

        println!("{}", local_value);
        output += local_value;
    }

    println!("{}", output);

    Ok(output as u64)
}


fn parse_histories(file_name: &str) -> Result<Vec<Vec<i64>>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let mut output = Vec::new();

    for l in lines {
        let Ok(line) = l else {
            return Err("Error in reading line from file");
        };

        let chars_vec = line.chars().collect::<Vec<char>>();
        let mut chars = chars_vec.iter().peekable();
        let mut numbers = Vec::new();

        loop {
            let Some(n) = parse_num(&mut chars, true) else {
                break;
            };

            numbers.push(n);            
        }

        output.push(numbers);
    }


    Ok(output)
}