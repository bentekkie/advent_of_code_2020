extern crate lazy_static;
extern crate regex;
extern crate snafu;


use lazy_static::lazy_static;
use regex::Regex;
use snafu::{OptionExt, ResultExt, Snafu};
use std::fs::{self};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    Ok(read_passports("../i")?.iter().flatten().count())
}

fn part2() -> Result<usize> {
    Ok(read_passports("../i")?
        .iter()
        .flatten()
        .map(Passport::is_valid)
        .flatten()
        .count())
}

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(line: &str) -> Result<Passport> {
        lazy_static! {
            static ref BYR: Regex = Regex::new(r"byr:([^ ]*)").unwrap();
            static ref IYR: Regex = Regex::new(r"iyr:([^ ]*)").unwrap();
            static ref EYR: Regex = Regex::new(r"eyr:([^ ]*)").unwrap();
            static ref HGT: Regex = Regex::new(r"hgt:([^ ]*)").unwrap();
            static ref HCL: Regex = Regex::new(r"hcl:([^ ]*)").unwrap();
            static ref ECL: Regex = Regex::new(r"ecl:([^ ]*)").unwrap();
            static ref PID: Regex = Regex::new(r"pid:([^ ]*)").unwrap();
        }

        let get_capture = |r: &Regex| -> Result<String> {
            Ok(r.captures(line.trim())
                .context(RegexMatch {
                    line,
                    regex: r.as_str(),
                })?
                .get(1)
                .context(RegexMatch {
                    line,
                    regex: r.as_str(),
                })?
                .as_str()
                .to_string())
        };

        Ok(Passport {
            byr: get_capture(&BYR)?,
            iyr: get_capture(&IYR)?,
            eyr: get_capture(&EYR)?,
            hgt: get_capture(&HGT)?,
            hcl: get_capture(&HCL)?,
            ecl: get_capture(&ECL)?,
            pid: get_capture(&PID)?,
        })
    }
}

impl Passport {
    fn is_valid(&self) -> Result<bool> {
        Ok((1920..=2002).contains(&self.byr.parse::<i32>().context(ParseInt {})?)
            && (2010..=2020).contains(&self.iyr.parse::<i32>().context(ParseInt {})?)
            && (2020..=2030).contains(&self.eyr.parse::<i32>().context(ParseInt {})?)
            && self.hgt_valid()?
            && self.hcl_valid()?
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .any(|&s| self.ecl == s)
            && self.pid.len() == 9
            && self.pid.parse::<i32>().is_ok())
    }
    fn hgt_valid(&self) -> Result<bool> {
        if self.hgt.ends_with("cm") {
            let num = self.hgt[0..self.hgt.len() - 2]
                .parse::<i32>()
                .context(ParseInt {})?;
            Ok(150 <= num && num <= 193)
        } else if self.hgt.ends_with("in") {
            let num = self.hgt[0..self.hgt.len() - 2]
                .parse::<i32>()
                .context(ParseInt {})?;
            Ok(59 <= num && num <= 76)
        } else {
            Ok(false)
        }
    }
    fn hcl_valid(&self) -> Result<bool> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[0-9a-f]{6}").unwrap();
        }
        Ok(self.hcl.starts_with("#") && self.hcl[1..].len() == 6 && RE.is_match(&self.hcl[1..]))
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
    #[snafu(display("Could not parse regex for line {} and regex {}", line, regex))]
    RegexMatch { line: String, regex: String },
    #[snafu(display("Could not open parse int: {}", source))]
    ParseInt { source: ParseIntError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_passports<P>(filename: P) -> Result<Vec<Result<Passport>>>
where
    P: AsRef<Path>,
{
    Ok(fs::read_to_string(&filename)
        .context(OpenFile {
            filename: filename.as_ref(),
        })?
        .split("\r\n\r\n")
        .map(|s| s.to_string().replace("\r\n", " ").parse::<Passport>())
        .collect())
}
