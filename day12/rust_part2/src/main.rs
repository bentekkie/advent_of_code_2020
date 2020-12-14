extern crate snafu;

use snafu::{ResultExt, Snafu};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::ops::{Add, Mul};

fn main() {
    println!("Part2: {:?}", part2());
}

fn part2() -> Result<i32> {
    Ok(read_lines("../i")?
        .flatten()
        .map(|s| s.parse::<Command>())
        .fold(Ok(Ship::new()), |ship, cmd| ship?.sail(cmd?))?
        .dist())
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

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn turn(self, deg: i32) -> Point {
        match (deg + 360) % 360 {
            90 => Point {
                x: -self.y,
                y: self.x,
            },
            180 => Point {
                x: -self.x,
                y: -self.y,
            },
            270 => Point {
                x: self.y,
                y: -self.x,
            },
            _ => self,
        }
    }
}

impl Add for Point {
type Output = Point;
fn add(self, other: Point) -> Point {
    Point {
        x:self.x + other.x,
        y:self.y + other.y
    }
 }
}
impl Mul<i32> for Point {
type Output = Point;
    fn mul(self, other: i32) -> Point { 
        Point {
            x:self.x * other,
            y:self.y * other
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
    loc: Point,
    waypoint: Point,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            loc: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        }
    }
    fn dist(&self) -> i32 {
        self.loc.x.abs() + self.loc.y.abs()
    }

    fn sail(&self, Command { action, value}: Command) -> Result<Ship> {
        let &Ship {loc, waypoint} = self;
        match action {
            Action::N => Ok(Ship {
                loc,
                waypoint: Point {
                    x: waypoint.x,
                    y: waypoint.y + value,
                },
            }),
            Action::S => Ok(Ship {
                loc,
                waypoint: Point {
                    x: waypoint.x,
                    y: waypoint.y - value,
                },
            }),
            Action::E => Ok(Ship {
                loc,
                waypoint: Point {
                    x: waypoint.x + value,
                    y: waypoint.y,
                },
            }),
            Action::W => Ok(Ship {
                loc,
                waypoint: Point {
                    x: waypoint.x - value,
                    y: waypoint.y,
                },
            }),
            Action::L => Ok(Ship {
                loc,
                waypoint: waypoint.turn(value),
            }),
            Action::R => Ok(Ship {
                loc,
                waypoint: waypoint.turn(-value),
            }),
            Action::F => Ok(Ship {
                loc: loc + waypoint * value,
                waypoint,
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
