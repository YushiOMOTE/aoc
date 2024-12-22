use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use reqwest::blocking;
use std::str::FromStr;
use std::ops::{Add, Div, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2d {
    pub x: isize,
    pub y: isize,
}

impl Vec2d {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_tuple(t: (isize, isize)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl From<(isize, isize)> for Vec2d {
    fn from(t: (isize, isize)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Rem for Vec2d {
    type Output = Vec2d;

    fn rem(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x % other.x,
            y: self.y % other.y,
        }
    }
}

impl Div<isize> for Vec2d {
    type Output = Vec2d;

    fn div(self, other: isize) -> Vec2d {
        Vec2d {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

pub fn fetch_input(day: usize) -> String {
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let session_token =
        std::env::var("AOC_SESSION").expect("missing AOC_SESSION environment variable");

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

pub fn parse_as_rows<T: FromStr>(input: &str) -> Vec<Vec<T>> {
    // Initialize a vector to store rows
    let mut rows: Vec<Vec<T>> = Vec::new();

    // Process each line
    for line in input.lines() {
        // Split the line into tokens and parse them into integers
        let row: Vec<T> = line
            .split_whitespace()
            .filter_map(|token| token.parse().ok())
            .collect();
        rows.push(row);
    }

    rows
}
