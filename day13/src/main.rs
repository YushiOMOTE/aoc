use aoc24_util::fetch_input;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Sub;

#[derive(Debug)]
struct Button {
    shift: Coord,
    cost: usize,
}

impl Button {
    fn new(shift: Coord, cost: usize) -> Self {
        Self { shift, cost }
    }
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
}

impl Machine {
    fn new(a: Button, b: Button) -> Self {
        Self { a, b }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

struct Context {
    machine: Machine,
    cost_record: HashMap<Coord, usize>,
}

impl Context {
    fn new(machine: Machine) -> Self {
        Self {
            machine,
            cost_record: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Counter {
    a: usize,
    b: usize,
}

impl Counter {
    fn new() -> Self {
        Self { a: 0, b: 0 }
    }

    fn press_a(self) -> Self {
        Self {
            a: self.a + 1,
            b: self.b,
        }
    }

    fn press_b(self) -> Self {
        Self {
            a: self.a,
            b: self.b + 1,
        }
    }
}

fn min_cost_sub(ctx: &mut Context, coord: Coord, counter: Counter) -> usize {
    let c = coord;

    if counter.a > 100 || counter.b > 100 {
        return std::usize::MAX;
    } else if c.x == 0 && c.y == 0 {
        return 0;
    } else if c.x < 0 || c.y < 0 {
        return std::usize::MAX;
    } else if let Some(existing) = ctx.cost_record.get(&c) {
        return *existing;
    }

    let cost_a = min_cost_sub(ctx, c - ctx.machine.a.shift, counter.press_a())
        .saturating_add(ctx.machine.a.cost);
    let cost_b = min_cost_sub(ctx, c - ctx.machine.b.shift, counter.press_b())
        .saturating_add(ctx.machine.b.cost);

    let cost = cost_a.min(cost_b);

    ctx.cost_record.insert(c, cost);

    cost
}

fn min_cost(ctx: &mut Context, coord: Coord) -> usize {
    min_cost_sub(ctx, coord, Counter::new())
}

fn parse_spec_line(text: &str, delim: char) -> (isize, isize) {
    text.split(':')
        .skip(1)
        .map(|shifts| {
            let (x, y) = shifts
                .trim()
                .split(',')
                .map(|shift| {
                    shift
                        .split(delim)
                        .skip(1)
                        .map(|v| v.parse::<isize>().unwrap())
                })
                .flatten()
                .collect_tuple()
                .unwrap();

            (x, y)
        })
        .last()
        .unwrap()
}

fn parse(text: &str) -> Vec<(Machine, Coord)> {
    let text: Vec<_> = text.split('\n').collect();

    text.chunks(4)
        .map(|lines| {
            // println!("{:?}", lines);

            assert!(lines[0].find("Button A:").is_some());
            assert!(lines[1].find("Button B:").is_some());
            assert!(lines[2].find("Prize:").is_some());
            assert!(lines.len() == 3 || lines[3].is_empty());

            let button_a_spec = parse_spec_line(lines[0], '+');
            let button_b_spec = parse_spec_line(lines[1], '+');
            let target = parse_spec_line(lines[2], '=');

            (
                Machine::new(
                    Button::new(Coord::new(button_a_spec.0, button_a_spec.1), 3),
                    Button::new(Coord::new(button_b_spec.0, button_b_spec.1), 1),
                ),
                Coord::new(target.0, target.1),
            )
        })
        .collect()
}

fn resolve(specs: Vec<(Machine, Coord)>) -> usize {
    specs
        .into_iter()
        .map(|(machine, target)| {
            let mut ctx = Context::new(machine);
            min_cost(&mut ctx, target)
        })
        .filter(|c| *c != std::usize::MAX)
        .sum()
}

fn main() {
    let text = fetch_input(13);
    //     let text = "Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400

    // Button A: X+26, Y+66
    // Button B: X+67, Y+21
    // Prize: X=12748, Y=12176

    // Button A: X+17, Y+86
    // Button B: X+84, Y+37
    // Prize: X=7870, Y=6450

    // Button A: X+69, Y+23
    // Button B: X+27, Y+71
    // Prize: X=18641, Y=10279";

    let specs = parse(&text);

    println!("{}", resolve(specs));
}
