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

/*
3   4
4   3
2   5
1   3
3   9
3   3
 */

fn main() {
    let input = fetch_input(1);
    let columns = parse_as_columns(&input);

    // println!("{}", resolve(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]));
    println!("{}", resolve(&columns[0], &columns[1]));
}

