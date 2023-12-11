use crate::days::Part;
use crate::days::{read_lines, parse_num};

use std::fs::File;
use std::io::{self};


pub fn run(file_name: &str, part: Part) -> Result<u64, &'static str> {
  match part {
      Part::P1 => part1(file_name),
      Part::P2 => part2(file_name),
  }
}


fn part1(file_name: &str) -> Result<u64, &'static str> {
  let races = parse_races(file_name)?;
  let mut product = 1;

  for (time, dist) in races {
    // println!("race -- {} {}", time, dist);
    for start_speed in 1..time {
      let distance = start_speed * (time - start_speed);

      // println!("  - {} {}", start_speed, distance);

      if distance > dist {
        let amount = (time - 1) - (start_speed - 1) * 2;
        // println!("  - done {}", amount);
        product *= amount;
        break;
      }
    }
  }

  Ok(product as u64)
}


fn part2(file_name: &str) -> Result<u64, &'static str> {
  let races = parse_races(file_name)?;
  let races = combine_races(races);
  let mut product = 1;
  // 47986609

  for (time, dist) in &races {
    // println!("race -- {} {}", time, dist);
    let mut start_speed = 1;
    let mut left = 1;
    let mut right = *time - 1;

    while left <= right {
      start_speed = (left + right) / 2;
      let distance_less = (start_speed - 1) * (*time - start_speed);
      let distance_more = (start_speed + 1) * (*time - start_speed);
  
        // println!("  - {} {}", start_speed, distance);

        if distance_less > *dist {
          right = start_speed - 1;
          continue;
        }

        if distance_more < *dist {
          left = start_speed + 1;
          continue;
        }

        // println!("  - done");
        break;
    }

    for (time, dist) in &races {
      // println!("race -- {} {}", time, dist);
      for start_speed in start_speed..*time {
        let distance = start_speed * (*time - start_speed);
  
        // println!("  - {} {}", start_speed, distance);
  
        if distance > *dist {
          let amount = (*time - 1) - (start_speed - 1) * 2;
          // println!("  - done {}", amount);
          product *= amount;
          break;
        }
      }
    }
  }

  Ok(product as u64)
}


fn combine_races(races: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
  let mut output = Vec::new();
  let mut t = 0;
  let mut d = 0;

  let base = 10 as u64;

  for (time, dist) in  races {
    t = (t * base.pow(length(time, 10) as u32)) + time;
    d = (d * base.pow(length(dist, 10) as u32)) + dist;
  }

  output.push((t, d));

  output
}



fn parse_races(file_name: &str) -> Result<Vec<(u64, u64)>, &'static str> {
  let Ok(mut lines) = read_lines(file_name) else {
      return Err("Failed to read file");
  };

  let Some(times) = parse_line(&mut lines) else {
    return Err("Failed to parse times");
  };

  let Some(distances) = parse_line(&mut lines) else {
    return Err("Failed to parse distances");
  };

  if times.len() != distances.len() {
    return Err("mistmatch");
  }

  let mut output = Vec::new();

  for i in 0..(times.len()) {
    output.push((times[i], distances[i]));
  }

  Ok(output)
}


fn parse_line(lines: &mut io::Lines<io::BufReader<File>>) -> Option<Vec<u64>>{
  let Ok(line) = lines.next()? else {
      return None;
  };
  let chars_vec = line.chars().collect::<Vec<char>>();
  let mut chars = chars_vec.iter().peekable();
  let mut seeds = Vec::new();

  loop {
      let Some(n) = parse_num(&mut chars, true) else {
          break;
      };

      seeds.push(n as u64);
  }

  Some(seeds)
}


fn length(n: u64, base: u64) -> u64 {
  let mut power = base;
  let mut count = 1;
  while n >= power {
      count += 1;
      if let Some(new_power) = power.checked_mul(base) {
          power = new_power;
      } else {
          break;
      }
  }
  count
}