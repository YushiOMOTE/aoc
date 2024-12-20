use aoc24_util::fetch_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn resolve(map: Vec<String>, map_size: (isize, isize)) -> usize {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, l) in map.iter().enumerate() {
        for (x, ch) in l.chars().enumerate() {
            if !ch.is_alphabetic() && !ch.is_digit(10) {
                continue;
            }

            antennas
                .entry(ch)
                .or_insert_with(Vec::new)
                .push((x as isize, y as isize));
        }
    }

    // println!("antennas: {:#?}", antennas);

    let antinodes: HashSet<_> = antennas
        .iter()
        .map(|(_, locations)| {
            locations.iter().permutations(2).map(|location_pair| {
                let (x1, y1) = location_pair[0];
                let (x2, y2) = location_pair[1];

                (x2 + (x2 - x1), y2 + (y2 - y1))
            })
        })
        .flatten()
        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < map_size.0 && *y < map_size.1)
        .collect();

    antinodes.len()
}

fn main() {
    let text = fetch_input(8);

    let map: Vec<String> = text.trim().split('\n').map(|s| s.into()).collect();
    let map_size = (map[0].len() as isize, map.len() as isize);

    println!("{}", resolve(map, map_size));
}
