use aoc24_util::fetch_input;
use regex::Regex;

fn resolve(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap())
        .sum()
}

fn main() {
    let input = fetch_input(3);
    println!("{}", resolve(&input));
}
