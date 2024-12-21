use aoc24_util::fetch_input;
use itertools::Itertools;
use std::ops::{Add, Div, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2d {
    x: isize,
    y: isize,
}

impl Vec2d {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn from_tuple(t: (isize, isize)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Rem for Vec2d {
    type Output = Vec2d;

    fn rem(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Div<isize> for Vec2d {
    type Output = Vec2d;

    fn div(self, other: isize) -> Vec2d {
        Vec2d {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

struct Map {
    size: Vec2d,
}

impl Map {
    fn new(x: isize, y: isize) -> Self {
        Self {
            size: Vec2d::new(x, y),
        }
    }
}

#[derive(Debug)]
struct Robot {
    pos: Vec2d,
    vel: Vec2d,
}

impl Robot {
    fn new(pos: Vec2d, vel: Vec2d) -> Self {
        Self { pos, vel }
    }

    fn step(&mut self, map: &Map) {
        self.pos = ((self.pos + self.vel) + map.size) % map.size;
    }
}

fn parse(text: &str) -> Vec<Robot> {
    let specs: Vec<Vec2d> = text
        .trim()
        .split('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|exp| exp.split('=').skip(1))
                .flatten()
                .map(|value| {
                    Vec2d::from_tuple(
                        value
                            .split(',')
                            .map(|v| v.parse().unwrap())
                            .collect_tuple()
                            .unwrap(),
                    )
                })
        })
        .flatten()
        .collect();

    specs
        .iter()
        .step_by(2)
        .zip(specs.iter().skip(1).step_by(2))
        .map(|(p, v)| Robot::new(*p, *v))
        .collect()
}

fn resolve(mut robots: Vec<Robot>, map: Map) -> usize {
    for _ in 0..100 {
        for robot in &mut robots {
            robot.step(&map);
        }
    }

    let mut quadrants = vec![vec![0, 0], vec![0, 0]];
    let center = map.size / 2;

    for robot in &robots {
        if robot.pos.x == center.x || robot.pos.y == center.y {
            continue;
        }

        quadrants[(robot.pos.x / (center.x + 1)) as usize]
            [(robot.pos.y / (center.y + 1)) as usize] += 1;
    }

    quadrants.iter().flatten().fold(1, |m, v| m * v)
}

fn main() {
    let map = Map::new(101, 103);
    let text = fetch_input(14);

    let robots = parse(&text);
    let safety_factor = resolve(robots, map);

    println!("{}", safety_factor);
}
