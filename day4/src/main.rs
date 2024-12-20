use aoc24_util::fetch_input;

fn is_xmas(a: char, b: char, c: char, d: char) -> bool {
    match (a, b, c, d) {
        ('X', 'M', 'A', 'S') | ('S', 'A', 'M', 'X') => true,
        _ => false,
    }
}

fn scan<F: Fn(usize) -> usize, G: Fn(usize, usize, usize, usize) -> bool>(
    input: &str,
    width: usize,
    constraint: G,
    scope: F,
) -> usize {
    input
        .chars()
        .enumerate()
        .zip(input.chars().enumerate().skip(scope(1)))
        .zip(input.chars().enumerate().skip(scope(2)))
        .zip(input.chars().enumerate().skip(scope(3)))
        .filter(|(((a, b), c), d)| {
            constraint(a.0 / width, b.0 / width, c.0 / width, d.0 / width)
                && is_xmas(a.1, b.1, c.1, d.1)
        })
        .count()
}

fn same_line_constraint(a: usize, b: usize, c: usize, d: usize) -> bool {
    a == b && b == c && c == d
}

fn contiguous_lines_constraint(a: usize, b: usize, c: usize, d: usize) -> bool {
    a + 1 == b && b + 1 == c && c + 1 == d
}

// horizontal (forwards, backwards)
// -> target: i, i+1, i+2, i+3
// vertical (forwards, backwards)
// -> target: i, i+w, i+2*w, i+3*w
// diagonal (down) (forwards, backwards)
// -> target: i, i+(w+1), i+2(w+1), i+3(w+1) (
// diagonal (up) (forwards, backwards)
// -> target: i, i+(w-1), i+2(w-1), i+3(w-1)
fn scan_all(input: &str, width: usize) -> usize {
    scan(input, width, same_line_constraint, |i| i)
        + scan(input, width, contiguous_lines_constraint, |i| i * width)
        + scan(input, width, contiguous_lines_constraint, |i| {
            i * (width + 1)
        })
        + scan(input, width, contiguous_lines_constraint, |i| {
            i * (width - 1)
        })
}

fn main() {
    let text = fetch_input(4);

    let width = text.find('\n').unwrap();
    let text = text.replace('\n', "");

    println!("{}", scan_all(&text, width));
}
