use crate::days::Part;
use crate::days::{read_lines, parse_num};

use std::cmp::Ordering;
use std::cmp;


#[derive(Debug, Eq, PartialOrd, Ord, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug, Eq, PartialOrd, Ord, PartialEq, Copy, Clone)]
enum CardType {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [CardType; 5],
    hand_type: HandType,
    bid: u32,
    joker_count: u32
}


pub fn run(file_name: &str, part: Part) -> Result<u32, &'static str> {
  match part {
      Part::P1 => part1(file_name, false),
      Part::P2 => part1(file_name, true),
  }
}


fn part1(file_name: &str, use_joker: bool) -> Result<u32, &'static str> {
    let hands = parse_hands(file_name, use_joker)?;
    let mut output = 0;
    let length = hands.len() as u32;

    for (i, h) in hands.iter().enumerate() {
        println!("{:?}", h);
        output += h.bid * (length - i as u32);
    }
  
    Ok(output)
}



fn parse_hands(file_name: &str, use_joker: bool) -> Result<Vec<Hand>, &'static str> {
    let Ok(lines) = read_lines(file_name) else {
        return Err("Failed to read file");
    };

    let mut output = Vec::new();
  
    for l in lines {
        let Ok(line) = l else {
            break;
        };

        let Some(hand) = parse_hand(&line, use_joker) else {
            return Err("Failed to parse hand");
        };

        output.push(hand);
    }

    output.sort();
  
    Ok(output)
}


fn parse_hand(line: &str, use_joker: bool) -> Option<Hand> {
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
    let mut hand: [CardType; 5] = [CardType::Two; 5];
    let mut joker_count = 0;

    for j in 0..5 {
        let (i, c) = match chars.peek() {
            Some('2') => (0, CardType::Two),
            Some('3') => (1, CardType::Three),
            Some('4') => (2, CardType::Four),
            Some('5') => (3, CardType::Five),
            Some('6') => (4, CardType::Six),
            Some('7') => (5, CardType::Seven),
            Some('8') => (6, CardType::Eight),
            Some('9') => (7, CardType::Nine),
            Some('T') => (8, CardType::Ten),
            Some('J') => (9, if use_joker { CardType::Joker } else { CardType::Jack } ),
            Some('Q') => (10, CardType::Queen),
            Some('K') => (11, CardType::King),
            Some('A') => (12, CardType::Ace),
            _ => return None,
        };
        chars.next();

        if c == CardType::Joker {
            joker_count += 1;
        }

        count[i as usize] += 1;
        hand[j as usize] = c;
    }

    let bid = parse_num(&mut chars, true)? as u32;
    let hand_type = determin_type(&count, joker_count);

    Some(Hand {
        cards: hand,
        hand_type,
        bid: bid,
        joker_count
    })
}


fn determin_type(count: &[u32; 13], joker_count: u32) -> HandType {
    let mut has_three = false;
    let mut has_two = false;

    for c in count {
        if *c == 5 {
            return HandType::FiveOfAKind;
        } else if *c == 4 {
            if joker_count == 1 {
                return HandType::FiveOfAKind;
            }
            return HandType::FourOfAKind;
        } else if *c == 3 {
            has_three = true;
            if joker_count == 2 {
                return HandType::FiveOfAKind;
            } else if joker_count == 1 {
                return HandType::FourOfAKind;
            } else if has_two {
                return HandType::FullHouse;
            }
        } else if *c == 2 {
            if joker_count == 3 {
                return HandType::FiveOfAKind
            } else if has_three {
                return HandType::FullHouse;
            } else if has_two {
                if joker_count == 1 {
                    return HandType::FullHouse;
                } else if joker_count == 2 {
                    return HandType::FourOfAKind;
                }
                return HandType::TwoPair;
            } else {
                has_two = true;
            }
        }
    }

    if has_two && has_three {
        return HandType::FullHouse;
    }

    if has_three {
        if joker_count == 1 {
            return HandType::FourOfAKind;
        } else if joker_count == 2 {
            return HandType::FiveOfAKind;
        }
        return HandType::ThreeOfAKind;
    }

    if has_two {
        if joker_count == 1 {
            return HandType::ThreeOfAKind;
        } else if joker_count == 3 {
            return HandType::FullHouse;
        }
        return HandType::OnePair;
    }

    if joker_count == 1 {
        return HandType::OnePair;
    }

    // 251129439 too low
    // 251333327 too high 

    return HandType::HighCard;
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