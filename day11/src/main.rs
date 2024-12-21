#![feature(linked_list_cursors)]

use aoc24_util::fetch_input;
use std::collections::LinkedList;

fn blink(stones: &mut LinkedList<usize>) {
    let mut cursor = stones.cursor_front_mut();

    while cursor.current().is_some() {
        let value = *cursor.current().unwrap();
        let digits = value.to_string().len();

        // print!("value {} ", value);

        if value == 0 {
            // println!("becomes 1");
            *cursor.current().unwrap() = 1;
        } else if digits % 2 == 0 {
            let divider = 10usize.pow(digits as u32 / 2);

            cursor.insert_after(value % divider);

            *cursor.current().unwrap() = value / divider;

            cursor.move_next();

            // println!("becomes {}, {}", value / divider, value % divider);
        } else {
            // println!("becomes {}" ,value * 2024);

            *cursor.current().unwrap() = value * 2024;
        }

        cursor.move_next();
    }
}

fn main() {
    let text = fetch_input(11);

    let mut stones: LinkedList<usize> = text
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    // println!("{:?}", stones);

    for _ in 0..25 {
        blink(&mut stones);
    }

    println!("{}", stones.len());
}
