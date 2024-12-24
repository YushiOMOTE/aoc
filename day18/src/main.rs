use aoc24_util::{fetch_input, Vec2d};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn new(w: usize, h: usize) -> Self {
        Self {
            map: vec![vec![true; w]; h],
        }
    }

    fn width(&self) -> isize {
        self.map[0].len() as isize
    }

    fn height(&self) -> isize {
        self.map.len() as isize
    }

    fn close(&mut self, x: isize, y: isize) {
        self.map[y as usize][x as usize] = false;
    }

    fn is_open(&self, pos: Vec2d) -> bool {
        self.map
            .get(pos.y as usize)
            .and_then(|line| line.get(pos.x as usize))
            .cloned()
            .unwrap_or(false)
    }

    fn debug(&self) {
        for line in &self.map {
            for open in line {
                let ch = if *open { '.' } else { '#' };
                print!("{}", ch);
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct State {
    pos: Vec2d,
    steps: usize,
    cost: usize,
}

impl State {
    fn new(pos: Vec2d, steps: usize, dist: usize) -> Self {
        Self {
            pos,
            steps,
            cost: steps + dist,
        }
    }
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

fn dist(a: Vec2d, b: Vec2d) -> usize {
    (a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2)).isqrt()
}

fn search(map: &Map) -> usize {
    let mut heap = BinaryHeap::new();
    let mut existings = HashMap::new();
    let start = Vec2d::new(0, 0);
    let goal = Vec2d::new(map.width() - 1, map.height() - 1);

    heap.push(State::new(start, 0, 0));

    while let Some(state) = heap.pop() {
        println!("{:?},{},{}", state.pos, state.cost, existings.len());

        if state.pos == goal {
            return state.cost;
        }

        for offset in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let offset = Vec2d::new(offset.0, offset.1);

            let new_pos = state.pos + offset;

            if !map.is_open(new_pos) {
                continue;
            }

            let new_steps = state.steps + 1;
            let new_dist = dist(goal, new_pos);
            let new_state = State::new(new_pos, new_steps, new_dist);

            if let Some(steps) = existings.get(&new_pos) {
                if *steps <= new_state.steps {
                    continue;
                }
            }
            existings.insert(new_pos, new_state.steps);

            heap.push(new_state);
        }
    }

    unreachable!("no such path")
}

fn resolve(text: &str, w: usize, h: usize, first_n: usize) -> usize {
    let mut map = Map::new(w, h);

    for (x, y) in text
        .split('\n')
        .map(|v| {
            v.split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .take(first_n)
    {
        map.close(x, y);
    }

    map.debug();

    search(&map)
}

fn main() {
    let text = fetch_input(18);

    println!("{}", resolve(&text, 71, 71, 1024));
}

#[test]
fn test_1() {
    let text = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    assert_eq!(22, resolve(text, 7, 7, 12));
}
