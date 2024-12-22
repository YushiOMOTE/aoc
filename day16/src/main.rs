use aoc24_util::fetch_input;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn parse(text: &str) -> Self {
        Self {
            map: text
                .split('\n')
                .map(|line| line.chars().collect())
                .collect(),
        }
    }

    fn find(&self, ch: char) -> Position {
        self.map
            .iter()
            .enumerate()
            .find_map(|(y, line)| {
                line.iter().enumerate().find_map(move |(x, c)| {
                    if *c == ch {
                        Some(Position::new(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn find_start(&self) -> Position {
        self.find('S')
    }

    fn find_goal(&self) -> Position {
        self.find('E')
    }

    fn get(&self, x: isize, y: isize) -> Option<&char> {
        self.map.get(y as usize).and_then(|l| l.get(x as usize))
    }

    fn visitable(&self, pos: Position) -> bool {
        self.get(pos.x, pos.y).map(|c| *c != '#').unwrap_or(false)
    }
}

#[derive(Debug, Clone, Eq)]
struct State {
    cost: usize,
    pos: Position,
    dir: Direction,
    visited: HashSet<Position>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl State {
    fn new(pos: Position, dir: Direction) -> Self {
        let mut visited = HashSet::new();

        visited.insert(pos);

        Self {
            cost: 0,
            pos,
            dir,
            visited,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn as_offset(&self) -> Position {
        match self {
            Self::North => Position::new(0, -1),
            Self::South => Position::new(0, 1),
            Self::East => Position::new(1, 0),
            Self::West => Position::new(-1, 0),
        }
    }

    fn turn_cost(&self, to: Direction) -> usize {
        match (self, to) {
            (Self::North, Self::South)
            | (Self::South, Self::North)
            | (Self::East, Self::West)
            | (Self::West, Self::East) => 2000,
            (x, y) if *x == y => 0,
            _ => 1000,
        }
    }
}

fn resolve(map: &Map) -> usize {
    let start = map.find_start();
    let goal = map.find_goal();

    let mut cache = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State::new(start, Direction::East));

    while let Some(mut state) = heap.pop() {
        if state.pos == goal {
            return state.cost;
        }

        if let Some(existing_cost) = cache.get(&state.pos) {
            if *existing_cost < state.cost {
                continue;
            }
        }

        cache.insert(state.pos, state.cost);

        for dir in [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ] {
            let offset = dir.as_offset();

            let new_pos = Position::new(state.pos.x + offset.x, state.pos.y + offset.y);

            if !map.visitable(new_pos) {
                // wall!
                continue;
            }

            if state.visited.contains(&new_pos) {
                // visited!
                continue;
            }
            state.visited.insert(new_pos);

            let mut forked_state = state.clone();

            let turn_cost = state.dir.turn_cost(dir);
            let move_cost = 1;

            forked_state.pos = new_pos;
            forked_state.cost = state.cost + turn_cost + move_cost;
            forked_state.dir = dir;

            heap.push(forked_state);
        }
    }

    unreachable!("no path found")
}

fn main() {
    let text = fetch_input(16);

    let map = Map::parse(&text);

    let cost = resolve(&map);

    println!("{}", cost);
}

#[test]
fn test_1() {
    let text = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    let map = Map::parse(&text);
    assert_eq!(resolve(&map), 11048);
}

#[test]
fn test_2() {
    let text = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    let map = Map::parse(&text);
    assert_eq!(resolve(&map), 7036);
}
