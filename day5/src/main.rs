use aoc24_util::fetch_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn evaluate(updates: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    updates
        .iter()
        .zip(updates.iter().skip(1))
        .all(|(a, b)| rules.get(a).filter(|s| s.contains(b)).is_some())
}

fn parse_rules_updates(text: &str) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for l in text.split('\n') {
        if l.contains('|') {
            let (k, v) = l
                .split('|')
                .map(|s| s.parse().unwrap())
                .collect_tuple::<(usize, usize)>()
                .unwrap();
            rules.entry(k).or_insert_with(HashSet::new).insert(v);
        } else if !l.is_empty() {
            updates.push(l.split(',').map(|v| v.parse().unwrap()).collect());
        }
    }

    (rules, updates)
}

fn main() {
    let text = fetch_input(5);

    let (rules, updates) = parse_rules_updates(&text);

    println!(
        "{}",
        updates
            .into_iter()
            .map(|u| if evaluate(&u, &rules) {
                u[u.len() / 2]
            } else {
                0
            })
            .sum::<usize>()
    );
}
