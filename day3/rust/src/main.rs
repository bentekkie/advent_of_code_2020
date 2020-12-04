extern crate lazy_static;
extern crate snafu;

use snafu::{ResultExt, Snafu};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    trees_in_path(3, 1)
}

fn part2() -> Result<usize> {
    Ok([
        trees_in_path(1, 1),
        trees_in_path(3, 1),
        trees_in_path(5, 1),
        trees_in_path(7, 1),
        trees_in_path(1, 2),
    ]
    .iter()
    .flatten()
    .product())
}

fn trees_in_path(right: usize, down: usize) -> Result<usize> {
    Ok(read_lines("../i")?
        .step_by(down)
        .map(|line| line.context(ReadLine {})?.parse::<ParsedLine>())
        .flatten()
        .enumerate()
        .map(|(i, line)| (i*right, line))
        .filter(|(i, line)| line.tree_at(i))
        .count())
}

#[derive(Debug)]
struct ParsedLine {
    line: String,
}
impl ParsedLine {
    fn tree_at(&self, i: &usize) -> bool {
        self.line.as_bytes()[i % self.line.len()] as char == '#'
    }
}
impl FromStr for ParsedLine {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        Ok(ParsedLine {
            line: line.to_string(),
        })
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    OpenFile {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not read line: {}", source))]
    ReadLine { source: std::io::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

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
