extern crate snafu;

use snafu::{OptionExt, ResultExt, Snafu};
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    read_lines("../i")?
        .flatten()
        .map(|l| bitstring(&l))
        .flatten()
        .max()
        .context(NoValidLines {})
}

fn part2() -> Result<usize> {
    let ids: Vec<usize> = read_lines("../i")?
        .flatten()
        .map(|l| bitstring(&l))
        .flatten()
        .collect();
    Ok((*ids.iter().min().context(NoValidLines {})? as i32
        ..=*ids.iter().max().context(NoValidLines {})? as i32)
        .chain(ids.iter().map(|x| -(*x as i32)))
        .sum::<i32>() as usize)
}

fn bitstring(s: &str) -> Result<usize> {
    usize::from_str_radix(
        s.replace(&['F', 'L'][..], "0")
            .replace(&['B', 'R'][..], "1")
            .as_str(),
        2,
    )
    .context(ParseInt {})
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
    #[snafu(display("Could not open parse int: {}", source))]
    ParseInt { source: ParseIntError },
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
