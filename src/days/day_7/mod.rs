use crate::days::Part;
use crate::days::{read_lines, parse_num};

use std::cmp::Ordering;
use std::cmp;


#[derive(Debug, Eq, PartialOrd, Ord, PartialEq)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [char; 5],
    hand_type: Type,
    bid: u32,
}


pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
  match part {
      Part::P1 => part1(file_name),
      Part::P2 => todo!(),
  }
}


fn part1(file_name: &str) -> Result<u32, &'static str> {
    let hands = parse_hands(file_name)?;
    let mut output = 0;
    let length = hands.len() as u32;

    for (i, h) in hands.iter().enumerate() {
        println!("{:?}", h);
        output += h.bid * (length - i as u32);
    }
  
    Ok(output)
}



fn parse_hands(file_name: &str) -> Result<Vec<Hand>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let mut output = Vec::new();
  
    for l in lines {
        let Ok(line) = l else {
            break;
        };

        let Some(hand) = parse_hand(&line) else {
            return Err("Failed to parse hand");
        };

        output.push(hand);
    }

    output.sort();
  
    Ok(output)
}


fn parse_hand(line: &str) -> Option<Hand> {
    let chars_vec = line.chars().collect::<Vec<char>>();
    let mut chars = chars_vec.iter().peekable();

    /*
     * 2 -> 0,
     * 3 -> 1,
     * ...
     * 9 -> 7,
     * T -> 8,
     * J -> 9,
     * Q -> 10,
     * K -> 11,
     * A -> 12
     */

    let mut count: [u32; 13] = [0; 13];
    let mut hand: [char; 5] = ['0'; 5];

    for j in 0..5 {
        let (i, c) = match chars.peek() {
            ch @ Some('2'..='9') => (**ch.unwrap() as u32 - '2' as u32, **ch.unwrap()),
            Some('T') => (8, 'T'),
            Some('J') => (9, 'J'),
            Some('Q') => (10, 'Q'),
            Some('K') => (11, 'K'),
            Some('A') => (12, 'A'),
            _ => return None,
        };
        chars.next();

        count[i as usize] += 1;
        hand[j as usize] = c;
    }

    let bid = parse_num(&mut chars, true)? as u32;
    let hand_type = determin_type(&count);

    Some(Hand {
        cards: hand,
        hand_type,
        bid: bid,
    })
}


fn determin_type(count: &[u32; 13]) -> Type {
    let mut has_three = false;
    let mut has_two = false;

    for c in count {
        if *c == 5 {
            return Type::FiveOfAKind;
        } else if *c == 4 {
            return Type::FourOfAKind;
        } else if *c == 3 {
            has_three = true;
            if has_two {
                return Type::FullHouse;
            }
        } else if *c == 2 {
            if has_three {
                return Type::FullHouse;
            } else if has_two {
                return Type::TwoPair;
            } else {
                has_two = true;
            }
        }
    }

    if has_two && has_three {
        return Type::FullHouse;
    }

    if has_three {
        return Type::ThreeOfAKind;
    }

    if has_two {
        return Type::OnePair;
    }

    return Type::HighCard;
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && (
            self.cards == other.cards
        )
    }
}