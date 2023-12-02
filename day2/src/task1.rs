use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const RED_LIMIT: usize = 12;
const GREEN_LIMIT: usize = 13;
const BLUE_LIMIT: usize = 14;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"Game (?<id>[0-9]+): (?<draws>.*)").unwrap();
    static ref BALL_RE: Regex = Regex::new(r"(?<number>[0-9]+) (?<color>[a-z]+)").unwrap();
    static ref LIMIT_MAP: HashMap<&'static str, usize> = HashMap::from([
        ("red", RED_LIMIT),
        ("green", GREEN_LIMIT),
        ("blue", BLUE_LIMIT)
    ]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn validate_draw(draw: &str) -> Option<bool> {
    for balls in draw.split(", ") {
        let cap = BALL_RE.captures(balls)?;
        let color = &cap["color"];
        let number = &cap["number"].parse::<usize>().ok()?;
        let limit = LIMIT_MAP.get(color)?;
        if number > limit {
            return Some(false);
        }
    }
    Some(true)
}

fn check_single_line(line: String) -> Option<usize> {
    let cap = LINE_RE.captures(&line)?;
    let game_id = cap["id"].parse::<usize>().ok()?;

    if cap["draws"]
        .split("; ")
        .map(validate_draw)
        .all(|x| x.is_some_and(|x| x))
    {
        Some(game_id)
    } else {
        Some(0)
    }
}

fn solve(filename: &str) -> usize {
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            sum += check_single_line(line).unwrap_or(0);
        }
    }

    sum
}

fn main() {
    const FILENAME: &str = "input.txt";
    let solution = solve(FILENAME);
    println!("{solution}")
}
