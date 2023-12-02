use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"Game (?<id>[0-9]+): (?<draws>.*)").unwrap();
    static ref BALL_RE: Regex = Regex::new(r"(?<number>[0-9]+) (?<color>[a-z]+)").unwrap();
}

const GREEN: &str = "green";
const BLUE: &str = "blue";
const RED: &str = "red";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn validate_draw(draw: &str) -> Option<(usize, usize, usize)> {
    let mut green = 0;
    let mut blue = 0;
    let mut red = 0;
    for balls in draw.split(", ") {
        let cap = BALL_RE.captures(&balls)?;
        let color = &cap["color"];
        let number = &cap["number"].parse::<usize>().ok()?;
        match color {
            GREEN => { green += number }
            BLUE => { blue += number }
            RED => { red += number }
            _ => {}
        }
    }

    Some((green, blue, red))
}

fn check_single_line(line: String) -> Option<usize> {
    let cap = LINE_RE.captures(&line)?;
    let mut green = 0;
    let mut blue = 0;
    let mut red = 0;

    let update = |old: &mut usize, new: usize| {
        if *old < new {
            *old = new;
        }
    };

    for (g, b, r) in cap["draws"].split("; ").map(validate_draw).flatten() {
        update(&mut green, g);
        update(&mut blue, b);
        update(&mut red, r);
    }

    Some(green * blue * red)
}

fn solve(filename: &str) -> usize {
    let mut sum = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                sum += check_single_line(line).unwrap_or_else(|| 0);
            }
        }
    }

    return sum;
}


fn main() {
    const FILENAME: &str = "input.txt";
    let solution = solve(FILENAME);
    println!("{solution}")
}
