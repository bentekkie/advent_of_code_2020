extern crate itertools;
extern crate snafu;

use itertools::Itertools;
use snafu::{OptionExt, ResultExt, Snafu};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    Ok(read_responses("../i")?
        .iter()
        .map(|responses| {
            responses
                .iter()
                .map(|response| response.chars())
                .flatten()
                .unique()
                .count()
        })
        .sum())
}

fn part2() -> Result<usize> {
    Ok(read_responses("../i")?
        .iter()
        .map(intersect_count)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum())
}

fn hashset(data: &String) -> HashSet<&u8> {
    data.as_bytes().iter().collect::<HashSet<_>>()
}

fn intersect_count(sets: &Vec<String>) -> Result<usize> {
    let mut iter = sets.iter().map(hashset);
    Ok(iter
        .next()
        .map(|set| iter.fold(set, |set1, set2| &set1 & &set2))
        .context(NoValidLines {})?
        .len())
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    OpenFile {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not find any valid lines"))]
    NoValidLines {},
}

type Result<T, E = Error> = std::result::Result<T, E>;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_responses<P>(filename: P) -> Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_to_string(&filename)
        .context(OpenFile {
            filename: filename.as_ref(),
        })?
        .split("\r\n\r\n")
        .map(|s| s.to_string().split("\r\n").map(|s| s.to_string()).collect())
        .collect())
}
