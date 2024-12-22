use aoc24_util::fetch_input;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    p: Vec<usize>,
    out: Vec<usize>,
}

impl Cpu {
    fn parse(text: &str) -> Self {
        let spec: Vec<_> = text.trim().split('\n').collect();

        let a = spec[0]
            .split(':')
            .last()
            .map(|v| v.trim().parse().unwrap())
            .unwrap();
        let b = spec[1]
            .split(':')
            .last()
            .map(|v| v.trim().parse().unwrap())
            .unwrap();
        let c = spec[2]
            .split(':')
            .last()
            .map(|v| v.trim().parse().unwrap())
            .unwrap();

        let p = spec[4]
            .split(':')
            .last()
            .map(|seq| {
                seq.trim()
                    .split(',')
                    .map(|v| v.to_string().parse().unwrap())
                    .collect()
            })
            .unwrap();

        Self {
            a,
            b,
            c,
            pc: 0,
            p,
            out: vec![],
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        if self.pc >= self.p.len() {
            return false;
        }

        match self.p[self.pc] {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => {
                if self.jnz() {
                    return true;
                }
            }
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            _ => unreachable!(),
        }

        self.pc += 2;

        true
    }

    fn read_literal(&self) -> usize {
        self.p[self.pc + 1]
    }

    fn read_combo(&self) -> usize {
        let value = self.p[self.pc + 1];

        match value {
            0 | 1 | 2 | 3 => value,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self) {
        print!("adv: {} / 2.pow({})", self.a, self.read_combo());
        self.a = self.a / (2usize.pow(self.read_combo() as u32));
        println!("-> {}", self.a);
    }

    fn bxl(&mut self) {
        print!("bxl: {} ^ {}", self.b, self.read_literal());
        self.b = self.b ^ self.read_literal();
        println!("-> {}", self.b);
    }

    fn bst(&mut self) {
        print!("bst: {} % 8", self.read_combo());
        self.b = self.read_combo() % 8;
        println!("-> {}", self.b);
    }

    fn jnz(&mut self) -> bool {
        if self.a != 0 {
            println!("jnz: jump to {} (a = {})", self.read_literal(), self.a);
            self.pc = self.read_literal();
            true
        } else {
            println!("jnz: no jump");
            false
        }
    }

    fn bxc(&mut self) {
        print!("bxc: {} ^ {}", self.b, self.c);
        self.b = self.b ^ self.c;
        println!("-> {}", self.b);
    }

    fn out(&mut self) {
        println!("out: {}", self.read_combo() % 8);
        self.out.push(self.read_combo() % 8);
    }

    fn bdv(&mut self) {
        print!("bdv: {} / 2.pow({})", self.a, self.read_combo());
        self.b = self.a / (2usize.pow(self.read_combo() as u32));
        println!("-> {}", self.b);
    }

    fn cdv(&mut self) {
        print!("cdv: {} / 2.pow({})", self.a, self.read_combo());
        self.c = self.a / (2usize.pow(self.read_combo() as u32));
        println!("-> {}", self.c);
    }
}

fn resolve1(text: &str) -> String {
    let mut cpu = Cpu::parse(text);
    cpu.run();
    cpu.out.iter().map(|v| v.to_string()).join(",")
}

fn resolve2(text: &str) -> usize {
    let cpu = Cpu::parse(text);

    'a: for i in 0.. {
        let mut c = cpu.clone();

        c.a = i;

        // println!("{}", i);

        while c.step() {
            if !cpu.p.starts_with(&c.out) {
                continue 'a;
            }
        }

        if cpu.p == c.out {
            return i;
        }
    }

    unreachable!()
}

#[test]
fn test_1() {
    let text = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    assert_eq!("4,6,3,5,6,3,5,2,1,0", resolve1(text));
}

#[test]
fn test_2() {
    let text = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    assert_eq!("0,3,5,4,3,0", resolve1(text));
}

#[test]
fn test_3() {
    let text = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    assert_eq!(117440, resolve2(text));
}

fn main() {
    let text = fetch_input(17);

    println!("part1: {}", resolve1(&text));

    // println!("part2: {}", resolve2(&text));
}
