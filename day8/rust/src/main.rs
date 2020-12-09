extern crate itertools;
extern crate petgraph;
extern crate snafu;

use snafu::{OptionExt, ResultExt, Snafu};
use std::collections::HashSet;
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
    run(read_lines("../i")?
        .flatten()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>>>()?)
    .map(|(acc, _)| acc)
    .context(InvalidLine {})
}

fn part2() -> Result<i32> {
    let instructions = read_lines("../i")?
        .flatten()
        .map(|s| s.parse())
        .collect::<Result<Vec<_>>>()?;
    (0..instructions.len())
        .filter_map(|i| maybe_swap(&instructions, i))
        .map(|is| run(is))
        .flatten()
        .filter(|(_, finished)| *finished)
        .last()
        .context(InvalidLine {})
        .map(|(acc, _)| acc)
}

fn maybe_swap(instructions: &Vec<Instruction>, i: usize) -> Option<Vec<Instruction>> {
    match instructions.get(i)? {
        Instruction {
            cmd: Command::Nop,
            val,
        } => {
            let mut new = instructions.to_vec();
            new[i] = Instruction {
                cmd: Command::Jmp,
                val: *val,
            };
            Some(new)
        }
        Instruction {
            cmd: Command::Jmp,
            val,
        } => {
            let mut new = instructions.to_vec();
            new[i] = Instruction {
                cmd: Command::Nop,
                val: *val,
            };
            Some(new)
        }
        Instruction {
            cmd: Command::Acc,
            val: _,
        } => None,
    }
}

fn run(instructions: Vec<Instruction>) -> Option<(i32, bool)> {
    let mut pc = 0;
    let mut acc = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&pc) {
        visited.insert(pc);
        if pc >= instructions.len() {
            return Some((acc, true));
        }
        match instructions.get(pc)? {
            Instruction {
                cmd: Command::Nop,
                val: _,
            } => {
                pc = pc + 1;
            }
            Instruction {
                cmd: Command::Jmp,
                val,
            } => {
                pc = ((pc as i32) + val) as usize;
            }
            Instruction {
                cmd: Command::Acc,
                val,
            } => {
                pc = pc + 1;
                acc = acc + val
            }
        };
    }
    Some((acc, false))
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

#[derive(Debug, Clone)]
enum Command {
    Jmp,
    Nop,
    Acc,
}

#[derive(Debug, Clone)]
struct Instruction {
    cmd: Command,
    val: i32,
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Instruction> {
        let parts = s.split(' ').collect::<Vec<_>>();
        let cmd = (match parts.first().context(InvalidLine {})? {
            &"jmp" => Some(Command::Jmp),
            &"nop" => Some(Command::Nop),
            &"acc" => Some(Command::Acc),
            _ => None,
        })
        .context(InvalidLine {})?;
        let val = parts
            .last()
            .context(InvalidLine {})?
            .parse()
            .context(ParseInt {})?;

        Ok(Instruction { cmd, val })
    }
}
