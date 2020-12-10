extern crate snafu;

use snafu::{OptionExt, ResultExt, Snafu};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<i64> {
    let nums = read_lines("../i")?
        .flatten()
        .map(|s| s.parse::<i64>().context(ParseInt {}))
        .collect::<Result<Vec<_>>>()?;
    Ok(nums[(25..nums.len())
        .filter(|&i| !two_sum(&nums[i - 25..i], nums[i]))
        .next()
        .context(InvalidLine {})?])
}

fn part2() -> Result<i64> {
    let nums = read_lines("../i")?
        .flatten()
        .map(|s| s.parse::<i64>().context(ParseInt {}))
        .collect::<Result<Vec<_>>>()?;
    let (low, high) = subarray_sum(&nums, 1212510616).context(InvalidLine {})?;
    Ok(nums[low..high].iter().min().context(InvalidLine {})?
        + nums[low..high].iter().max().context(InvalidLine {})?)
}

fn subarray_sum(nums: &[i64], target: i64) -> Option<(usize, usize)> {
    let mut low = 0;
    let mut sum = nums[0];
    for i in 1..nums.len() {
        while sum > target {
            sum -= nums[low];
            low += 1;
        }
        if sum == target {
            return Some((low, i));
        }
        sum += nums[i];
    }
    None
}

fn two_sum(nums: &[i64], target: i64) -> bool {
    let mut prev = HashSet::new();
    for x in nums {
        if let Some(_) = prev.get(&(target - x)) {
            return true;
        }
        prev.insert(x);
    }
    false
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not open file {}: {}", filename.display(), source))]
    OpenFile {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Could not find any valid lines"))]
    InvalidLine {},
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
