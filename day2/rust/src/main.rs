extern crate lazy_static;
extern crate regex;
extern crate snafu;

use lazy_static::lazy_static;
use regex::Regex;
use snafu::{OptionExt, ResultExt, Snafu};
use std::char::ParseCharError;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    Ok(read_lines("../i")?
        .map(|line| line.context(ReadLine {})?.parse())
        .flatten()
        .filter(ParsedLine::is_valid_part_1)
        .count())
}
fn part2() -> Result<usize> {
    Ok(read_lines("../i")?
        .map(|line| line.context(ReadLine {})?.parse())
        .flatten()
        .filter(ParsedLine::is_valid_part_2)
        .count())
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
        (self.password.as_bytes()[self.first_num - 1] as char == self.letter)
            ^ (self.password.as_bytes()[self.second_num - 1] as char == self.letter)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not parse regex for line {}", line))]
    RegexMatch { line: String },
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    OpenFile {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not read line: {}", source))]
    ReadLine { source: std::io::Error },
    #[snafu(display("Could not open parse int: {}", source))]
    ParseInt { source: ParseIntError },
    #[snafu(display("Could not parse char: {}", source))]
    ParseChar { source: ParseCharError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

impl FromStr for ParsedLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d*)-(\d*) (.): (.*)").unwrap();
        }
        let caps = RE.captures(line).context(RegexMatch { line })?;
        let first_num = caps
            .get(1)
            .context(RegexMatch { line })?
            .as_str()
            .parse()
            .context(ParseInt {})?;
        let second_num = caps
            .get(2)
            .context(RegexMatch { line })?
            .as_str()
            .parse()
            .context(ParseInt {})?;
        let letter = caps
            .get(3)
            .context(RegexMatch { line })?
            .as_str()
            .parse()
            .context(ParseChar {})?;
        let password = caps
            .get(4)
            .context(RegexMatch { line })?
            .as_str()
            .to_string();
        Ok(ParsedLine {
            first_num,
            second_num,
            letter,
            password,
        })
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename).context(OpenFile {
        filename: filename.as_ref(),
    })?;
    Ok(io::BufReader::new(file).lines())
}
