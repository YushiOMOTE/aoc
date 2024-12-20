use itertools::Itertools; // 0.13.0
use aoc24_util::{fetch_input, parse_as_columns};

fn resolve(list1: &[usize], list2: &[usize]) -> usize {
    list1
        .iter()
        .cloned()
        .sorted()
        .zip(list2.iter().cloned().sorted())
        .map(|(v1, v2)| v1.abs_diff(v2))
        .sum()
}

fn main() {
    let input = fetch_input(1);
    let columns = parse_as_columns(&input);
    println!("{}", resolve(&columns[0], &columns[1]));
}
