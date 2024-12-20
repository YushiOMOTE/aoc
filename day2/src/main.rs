use aoc24_util::{fetch_input, parse_as_rows};

fn acceptable_range(v1: usize, v2: usize) -> bool {
    let diff = v1.abs_diff(v2);
    diff >= 1 && diff <= 3
}

fn resolve(reports: Vec<Vec<usize>>) -> usize {
    let items = [7, 6, 4, 2, 1];

    reports
        .iter()
        .filter(|row| {
            row.iter()
                .zip(row.iter().skip(1))
                .all(|(v1, v2)| v1 < v2 && acceptable_range(*v1, *v2))
                || row
                    .iter()
                    .zip(row.iter().skip(1))
                    .all(|(v1, v2)| v1 > v2 && acceptable_range(*v1, *v2))
        })
        .count()
}

fn main() {
    let text = fetch_input(2);
    let rows = parse_as_rows(&text);
    println!("{:#?}", rows);
    println!("{}", resolve(rows));
}
