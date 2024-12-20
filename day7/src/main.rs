use aoc24_util::fetch_input;
use itertools::Itertools;

fn resolve(test_value: usize, operands: &[usize]) -> bool {
    let mut remains: Vec<usize> = vec![test_value];

    for (i, &op) in operands.iter().enumerate().rev() {
        for remain in std::mem::take(&mut remains) {
            if i == 0 {
                if remain == op {
                    remains.push(remain);
                }
            } else {
                if remain >= op && remain % op == 0 {
                    remains.push(remain / op);
                }
                if remain > op {
                    remains.push(remain - op);
                }
            }
        }
    }

    !remains.is_empty()
}

fn main() {
    let text = fetch_input(7);

    let equations: Vec<(usize, Vec<usize>)> = text
        .trim()
        .split('\n')
        .map(|l| {
            let (test_value, operands) = l.split(':').collect_tuple().unwrap();
            let operands: Vec<usize> = operands
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (test_value.parse().unwrap(), operands)
        })
        .collect();

    let sum: usize = equations
        .iter()
        .map(|(test_value, operands)| {
            if resolve(*test_value, &operands) {
                *test_value
            } else {
                0
            }
        })
        .sum();

    println!("{}", sum);
}
