extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Option<usize> {
    if let Ok(lines) = read_lines("../i") {
        Some(
            lines
                .filter_map(Result::ok)
                .map(|line| parse_line(&line))
                .flatten()
                .filter(ParsedLine::is_valid_part_1)
                .count(),
        );
    }
    None
}
fn part2() -> Option<usize> {
    if let Ok(lines) = read_lines("../i") {
        Some(
            lines
                .filter_map(Result::ok)
                .map(|line| parse_line(&line))
                .flatten()
                .filter(ParsedLine::is_valid_part_2)
                .count(),
        );
    }
    None
}

#[derive(Debug)]
struct ParsedLine {
    first_num: usize,
    second_num: usize,
    letter: char,
    password: String,
}
impl ParsedLine {
    fn is_valid_part_1(&self) -> bool {
        let count = self.password.matches(self.letter).count();
        count >= self.first_num && count <= self.second_num
    }
    fn is_valid_part_2(&self) -> bool {
        let first_loc = self.password.as_bytes()[self.first_num - 1] as char == self.letter;
        let second_loc = self.password.as_bytes()[self.second_num - 1] as char == self.letter;
        first_loc ^ second_loc
    }
}

fn parse_line(s: &str) -> Option<ParsedLine> {
    let r = Regex::new(r"([^-]*)-([^-]*) (.): (.*)").unwrap();
    let caps = r.captures(s)?;
    let first_num = caps.get(1)?.as_str().parse().ok()?;
    let second_num = caps.get(2)?.as_str().parse().ok()?;
    let letter = caps.get(3)?.as_str().parse().ok()?;
    let password = caps.get(4)?.as_str().parse().ok()?;
    Some(ParsedLine {
        first_num,
        second_num,
        letter,
        password,
    })
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
