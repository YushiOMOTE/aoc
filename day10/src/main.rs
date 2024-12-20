use aoc24_util::fetch_input;
use std::collections::HashSet;

struct Map {
    map: Vec<Option<Location>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Self {
        let width = input.find('\n').unwrap();
        let map: Vec<_> = input
            .trim()
            .split('\n')
            .enumerate()
            .map(|(y, s)| {
                s.chars().enumerate().map(move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(Location::new(
                            x as isize,
                            y as isize,
                            c.to_string().parse().unwrap(),
                        ))
                    }
                })
            })
            .flatten()
            .collect();
        let height = map.len() / width;

        Self { map, width, height }
    }

    fn get(&self, x: isize, y: isize) -> Option<Location> {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            self.map[x as usize + y as usize * self.width]
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Location {
    x: isize,
    y: isize,
    level: usize,
}

impl Location {
    fn new(x: isize, y: isize, level: usize) -> Self {
        Self { x, y, level }
    }
}

fn score(map: &Map, trailhead: Location) -> usize {
    let mut locations = HashSet::new();

    locations.insert(trailhead);

    for _ in 0..9 {
        for current_location in std::mem::take(&mut locations) {
            for xdiff in [-1, 0, 1] {
                for ydiff in [-1, 0, 1] {
                    if xdiff * ydiff != 0 {
                        continue;
                    }

                    if let Some(next_location) =
                        map.get(current_location.x + xdiff, current_location.y + ydiff)
                    {
                        //print!("{:?} <- {:?}", next_location, current_location);
                        if next_location.level == current_location.level + 1 {
                            //println!("*");
                            locations.insert(next_location);
                        } else {
                            //println!();
                        }
                    }
                }
            }
        }
    }

    println!("trailhead: {:?}, score: {}", trailhead, locations.len());

    locations.len()
}

fn resolve(map: Map) -> usize {
    let trailheads: Vec<_> = map
        .map
        .iter()
        .filter_map(|v| *v)
        .filter(|location| location.level == 0)
        .collect();

    trailheads.iter().map(|t| score(&map, *t)).sum()
}

fn main() {
    let text = fetch_input(10);

    let map = Map::parse(&text);

    println!("{}", resolve(map));
}
