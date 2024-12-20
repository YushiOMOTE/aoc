use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::blocking;
use std::str::FromStr;

pub fn fetch_input(day: usize) -> String {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let session_token =
        std::env::var("AOC_SESSION").expect("Missing AOC_SESSION environment variable");

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!("session={}", session_token)).unwrap(),
    );

    let client = blocking::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .expect("coudln't request input");

    if response.status().is_success() {
        response.text().expect("couldn't fetch input as text")
    } else {
        panic!("request failed with status: {}", response.status());
    }
}

pub fn parse_as_columns<T: FromStr>(input: &str) -> Vec<Vec<T>> {
    let mut columns: Vec<Vec<T>> = Vec::new();

    // Process each line
    for line in input.lines() {
        // Split the line into tokens
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // Ensure the columns vector has enough capacity
        while columns.len() < tokens.len() {
            columns.push(Vec::new());
        }

        // Parse and store each token in the appropriate column
        for (i, token) in tokens.iter().enumerate() {
            if let Ok(value) = token.parse() {
                columns[i].push(value);
            }
        }
    }

    columns
}
