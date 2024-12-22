use aoc24_util::{fetch_input, Vec2d};

struct Map {
    map: Vec<Vec<Option<Object>>>,
}

impl Map {
    fn get(&self, p: Vec2d) -> Option<Object> {
        self.map[p.y as usize][p.x as usize].clone()
    }

    fn set(&mut self, p: Vec2d, value: Option<Object>) {
        self.map[p.y as usize][p.x as usize] = value;
    }

    fn move_object(&mut self, from: Vec2d, to: Vec2d) {
        if let Some(object) = self.get(from) {
            self.set(from, None);
            self.set(to, Some(object));
        }
    }

    fn try_move_objects(&mut self, target: Vec2d, offset: Vec2d) -> bool {
        let object = self.get(target).expect("target must be valid object");

        match object.ty {
            '#' => false,
            '@' | 'O' => {
                let movable = if self.get(target + offset).is_some() {
                    self.try_move_objects(target + offset, offset)
                } else {
                    true
                };

                if movable {
                    self.move_object(target, target + offset);
                }

                movable
            }
            _ => unreachable!(),
        }
    }

    fn score(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter().enumerate().map(move |(x, v)| match v {
                    Some(o) if o.ty == 'O' => y * 100 + x,
                    _ => 0,
                })
            })
            .flatten()
            .sum()
    }

    fn debug(&self) {
        for line in &self.map {
            for obj in line {
                match obj {
                    Some(obj) => print!("{}", obj.ty),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct Object {
    ty: char,
}

struct Robot {
    instructions: Vec<char>,
    index: usize,
    pos: Vec2d,
}

impl Robot {
    fn new(pos: Vec2d, instructions: Vec<char>) -> Self {
        Self {
            pos,
            instructions,
            index: 0,
        }
    }

    fn step(&mut self, map: &mut Map) -> bool {
        if self.index >= self.instructions.len() {
            return false;
        }

        let direction = match self.instructions[self.index] {
            '<' => (-1, 0),
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            _ => unreachable!(),
        };

        let offset = direction.into();

        if map.try_move_objects(self.pos, offset) {
            self.pos = self.pos + offset;
        }

        self.index += 1;

        true
    }
}

fn parse(text: &str) -> (Map, Robot) {
    let mut sections = text.split("\n\n");
    let mut robot_pos = Vec2d::new(0, 0);
    let map = sections
        .next()
        .unwrap()
        .split('\n')
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, v)| {
                    let pos = (x as isize, y as isize).into();

                    if v == '@' {
                        robot_pos = pos;
                    }

                    match v {
                        '.' => None,
                        '#' | '@' | 'O' => Some(Object { ty: v }),
                        v => unreachable!("{}", v),
                    }
                })
                .collect()
        })
        .collect();
    let instructions = sections
        .next()
        .unwrap()
        .chars()
        .filter(|v| *v != '\n')
        .collect();

    (Map { map }, Robot::new(robot_pos, instructions))
}

fn resolve(text: &str) -> usize {
    let (mut map, mut robot) = parse(&text);

    map.debug();

    while robot.step(&mut map) {}

    map.debug();

    map.score()
}

fn main() {
    let text = fetch_input(15);
    println!("{}", resolve(&text));
}

#[test]
fn test_1() {
    let text = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    assert_eq!(10092, resolve(&text));
}

#[test]
fn test_2() {
    let text = "########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

<^^>>>vv<v>>v<<";

    assert_eq!(2028, resolve(&text));
}
