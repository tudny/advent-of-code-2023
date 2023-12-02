use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"Game (?<id>[0-9]+): (?<draws>.*)").unwrap();
    static ref BALL_RE: Regex = Regex::new(r"(?<number>[0-9]+) (?<color>[a-z]+)").unwrap();
}

const ACCEPTABLE_COLORS: [&str; 3] = ["green", "blue", "red"];

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn validate_draw(draw: &str) -> Option<HashMap<&str, usize>> {
    let mut result = HashMap::from(ACCEPTABLE_COLORS.map(|c| (c, 0)));
    for balls in draw.split(", ") {
        let cap = BALL_RE.captures(balls)?;
        let color = &cap["color"];
        let number = &cap["number"].parse::<usize>().ok()?;
        if let Some(current) = result.get_mut(color) {
            *current += number;
        }
    }

    Some(result)
}

fn check_single_line(line: &str) -> Option<usize> {
    let cap = LINE_RE.captures(line)?;

    let mut current_scores = HashMap::from(ACCEPTABLE_COLORS.map(|c| (c, 0)));

    for draw_scores in cap["draws"].split("; ").flat_map(validate_draw) {
        for (color, current_score) in current_scores.iter_mut() {
            let score = draw_scores.get(color)?;
            *current_score = max(*current_score, *score);
        }
    }

    Some(current_scores.values().product())
}

fn solve(filename: &str) -> usize {
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            sum += check_single_line(&line).unwrap_or(0);
        }
    }

    sum
}

fn main() {
    const FILENAME: &str = "input.txt";
    let solution = solve(FILENAME);
    println!("{solution}")
}
