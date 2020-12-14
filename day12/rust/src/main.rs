extern crate snafu;

use snafu::{ResultExt, Snafu};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<i32> {
    Ok(read_lines("../i")?
        .flatten()
        .map(|s| s.parse::<Command>())
        .fold(Ok(Ship::new()), |ship, cmd| ship?.sail(cmd?))?
        .dist())
}

fn part2() -> Result<i64> {
    Ok(0)
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

#[derive(Debug, Clone, Copy)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl Action {
    fn turn(self, deg: i32) -> Result<Action> {
        let curr: Result<i32> = self.try_into();
        ((curr? + (deg + 360)) % 360).try_into()
    }
}

impl TryFrom<i32> for Action {
    type Error = Error;

    fn try_from(value: i32) -> Result<Action> {
        match value {
            0 => Ok(Action::N),
            90 => Ok(Action::E),
            180 => Ok(Action::S),
            270 => Ok(Action::W),
            _ => Err(Error::InvalidLine {}),
        }
    }
}
impl TryInto<i32> for Action {
    type Error = Error;

    fn try_into(self) -> Result<i32> {
        match self {
            Action::N => Ok(0),
            Action::S => Ok(180),
            Action::E => Ok(90),
            Action::W => Ok(270),
            _ => Err(Error::InvalidLine {}),
        }
    }
}

#[derive(Debug, Clone)]
struct Command {
    action: Action,
    value: i32,
}

#[derive(Debug, Clone)]
struct Ship {
    x: i32,
    y: i32,
    direction: Action,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            direction: Action::E,
        }
    }
    fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn sail(&self, Command { action, value}: Command) -> Result<Ship> {
        match action {
            Action::N => Ok(Ship {
                direction: self.direction,
                x: self.x,
                y: self.y + value,
            }),
            Action::S => Ok(Ship {
                direction: self.direction,
                x: self.x,
                y: self.y - value,
            }),
            Action::E => Ok(Ship {
                direction: self.direction,
                x: self.x + value,
                y: self.y,
            }),
            Action::W => Ok(Ship {
                direction: self.direction,
                x: self.x - value,
                y: self.y,
            }),
            Action::L => Ok(Ship {
                x: self.x,
                y: self.y,
                direction: self.direction.turn(-value)?,
            }),
            Action::R => Ok(Ship {
                x: self.x,
                y: self.y,
                direction: self.direction.turn(value)?,
            }),
            Action::F => self.sail(Command {
                action: self.direction,
                value,
            }),
        }
    }
}

impl FromStr for Action {
    type Err = Error;
    fn from_str(s: &str) -> Result<Action> {
        match s {
            "N" => Ok(Action::N),
            "S" => Ok(Action::S),
            "E" => Ok(Action::E),
            "W" => Ok(Action::W),
            "L" => Ok(Action::L),
            "R" => Ok(Action::R),
            "F" => Ok(Action::F),
            _ => Err(Error::InvalidLine {}),
        }
    }
}

impl FromStr for Command {
    type Err = Error;
    fn from_str(s: &str) -> Result<Command> {
        Ok(Command {
            action: s[0..1].parse()?,
            value: s[1..].parse().context(ParseInt {})?,
        })
    }
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
