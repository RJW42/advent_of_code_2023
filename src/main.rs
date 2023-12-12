mod days;

use days::*;
use days::Part::{P1, P2};


fn main() {
    let day = 13;

    let function = match day {
        1 => |p| day_1::run("inputs/day_1.txt", p),
        2 => |p| day_2::run("inputs/day_2.txt", p),
        3 => |p| day_3::run("inputs/day_3.txt", p),
        4 => |p| day_4::run("inputs/day_4.txt", p),
        5 => |p| day_5::run("inputs/day_5.txt", p),
        6 => |p| day_6::run("inputs/day_6.txt", p),
        7 => |p| day_7::run("inputs/day_7.txt", p),
        8 => |p| day_8::run("inputs/day_8.txt", p),
        9 => |p| day_9::run("inputs/day_9.txt", p),
        10 => |p| day_10::run("inputs/day_10.txt", p),
        11 => |p| day_11::run("inputs/day_11.txt", p),
        12 => |p| day_12::run("inputs/day_12.txt", p),
        13 => |p| day_13::run("inputs/day_13.txt", p),
        14 => |_p| todo!(),
        15 => |_p| todo!(),
        16 => |_p| todo!(),
        17 => |_p| todo!(),
        18 => |_p| todo!(),
        19 => |_p| todo!(),
        20 => |_p| todo!(),
        21 => |_p| todo!(),
        22 => |_p| todo!(),
        23 => |_p| todo!(),
        24 => |_p| todo!(),
        25 => |_p| todo!(),
        _ => |_| Result::Err("Invalid Day"),
    };

    let result = function(P1)
        .map_err(|e| format!("Failed P1: {e}"))
        .and_then(|r| {
            println!("P1 Result: {}", r);
            return function(P2)
                .map_err(|e| format!("Failed P2: {e}"))
                .map(|r| println!("P2 Results: {}", r));
        });

    match result {
        Ok(_) => (),
        Err(reason) => println!("{}", reason)
    };
}