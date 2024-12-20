use aoc24_util::fetch_input;

#[derive(Copy, Clone, Debug)]
enum Orientation {
    Up,
    Right,
    Left,
    Down,
}

impl Orientation {
    fn parse(ch: char) -> Self {
        match ch {
            '^' => Orientation::Up,
            '>' => Orientation::Right,
            '<' => Orientation::Left,
            'v' => Orientation::Down,
            _ => unreachable!("unexpected orientation"),
        }
    }

    fn turn(self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn new_pos(self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Orientation::Up => (pos.0, pos.1 - 1),
            Orientation::Right => (pos.0 + 1, pos.1),
            Orientation::Down => (pos.0, pos.1 + 1),
            Orientation::Left => (pos.0 - 1, pos.1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum State {
    Open,
    Visited,
    Block,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<State>>,
}

impl Map {
    fn is_out_of_map(&self, pos: (isize, isize)) -> bool {
        pos.0 < 0
            || pos.1 < 0
            || pos.1 >= self.map.len() as isize
            || pos.0 >= self.map[0].len() as isize
    }

    fn can_proceed(&self, pos: (isize, isize)) -> bool {
        !matches!(self.map[pos.1 as usize][pos.0 as usize], State::Block)
    }

    fn step(&mut self, pos: (isize, isize)) {
        self.map[pos.1 as usize][pos.0 as usize] = State::Visited;
    }

    fn count_visited(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|v| matches!(v, State::Visited)).count())
            .sum()
    }

    fn debug(&self) {
        for row in &self.map {
            for v in row {
                let ch = match v {
                    State::Open => '.',
                    State::Visited => 'X',
                    State::Block => '#',
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Guard {
    pos: (isize, isize),
    orientation: Orientation,
}

impl Guard {
    fn update(&mut self, map: &mut Map) -> bool {
        map.step(self.pos);
        
        let new_pos = self.orientation.new_pos(self.pos);

        if map.is_out_of_map(new_pos) {
            return false;
        }

        if map.can_proceed(new_pos) {
            self.pos = new_pos;
        } else {
            self.orientation = self.orientation.turn();
        }

        true
    }
}

fn parse(text: &str) -> (Map, Guard) {
    let mut guard = None;

    let map = text
        .split('\n')
        .enumerate()
        .map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, v)| match v {
                    '#' => State::Block,
                    '.' => State::Open,
                    '<' | '>' | 'v' | '^' => {
                        guard = Some(Guard {
                            pos: (x as isize, y as isize),
                            orientation: Orientation::parse(v),
                        });

                        State::Open
                    }
                    _ => unreachable!("unexpected input"),
                })
                .collect()
        })
        .collect();

    (Map { map }, guard.unwrap())
}

fn simulate(input: &str) -> usize {
    let (mut map, mut guard) = parse(input);

    while guard.update(&mut map) {
        // map.debug();
    }

    map.count_visited()
}

fn main() {
    // let text = "....#.....
// .........#
// ..........
// ..#.......
// .......#..
// ..........
// .#..^.....
// ........#.
// #.........
// ......#...";

    let text = fetch_input(6);
    
    println!("{}", simulate(&text));
}
