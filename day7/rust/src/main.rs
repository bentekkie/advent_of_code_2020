extern crate itertools;
extern crate petgraph;
extern crate snafu;

use itertools::Itertools;
use petgraph::algo::has_path_connecting;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use snafu::{OptionExt, ResultExt, Snafu};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};

fn main() {
    println!("Part1: {:?}", part1());
    println!("Part2: {:?}", part2());
}

fn part1() -> Result<usize> {
    let e = read_lines("../i")?
        .flatten()
        .map(edges)
        .flatten()
        .flatten()
        .collect::<Vec<_>>();
    let nodes = e
        .iter()
        .map(|(a, b)| vec![a, b])
        .flatten()
        .unique()
        .collect::<Vec<_>>();
    let mut g = DiGraph::<_, ()>::new();
    let mut indexes = HashMap::new();
    for node in nodes {
        indexes.insert(node, g.add_node(node));
    }
    for (a, b) in &e {
        g.add_edge(
            *indexes.get(&a).context(InvalidLine {})?,
            *indexes.get(&b).context(InvalidLine {})?,
            (),
        );
    }
    let start = indexes
        .remove(&"shiny gold".to_string())
        .context(InvalidLine {})?;
    Ok(indexes
        .iter()
        .filter(|(_, i)| has_path_connecting(&g, **i, start, None))
        .count())
}

fn part2() -> Result<usize> {
    let e = read_lines("../i")?
        .flatten()
        .map(edges)
        .flatten()
        .flatten()
        .collect::<Vec<_>>();
    let nodes = e
        .iter()
        .map(|(a, b)| vec![a, b])
        .flatten()
        .unique()
        .collect::<Vec<_>>();
    let mut g = DiGraph::<_, ()>::new();
    let mut indexes_bimap = BiMap::new();
    for node in nodes {
        let index = g.add_node(node);
        indexes_bimap.insert(node, index);
    }
    for (a, b) in &e {
        g.add_edge(
            *indexes_bimap.get_forward(&a).context(InvalidLine {})?,
            *indexes_bimap.get_forward(&b).context(InvalidLine {})?,
            (),
        );
    }
    let w: HashMap<_, _> = read_lines("../i")?
        .flatten()
        .map(weights)
        .flatten()
        .flatten()
        .collect();
    Ok(bags(
        "shiny gold".to_string(),
        &g,
        &w,
        &|s| indexes_bimap.get_forward(s).context(InvalidLine {}),
        &|s| Ok(indexes_bimap.get_reverse(&s).context(InvalidLine {})?),
    )?)
}

fn bags<'a, I: Fn(&String) -> Result<&'a NodeIndex>, R: Fn(NodeIndex) -> Result<&'a String>>(
    s: String,
    g: &DiGraph<&std::string::String, ()>,
    w: &HashMap<(String, String), usize>,
    get_i: &I,
    get_r: &R,
) -> Result<usize> {
    g.neighbors(*get_i(&s)?)
        .map(|x| {
            let right = get_r(x)?;
            Ok(w.get(&(s.clone(), right.to_string()))
                .context(InvalidLine {})?
                * (1 + bags(right.to_string(), g, w, get_i, get_r)?))
        })
        .sum::<Result<_>>()
}

fn edges(line: String) -> Result<Vec<(String, String)>> {
    let v = line
        .split(" contain ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let left = v.first().context(InvalidLine {})?;
    let right = v.last().context(InvalidLine {})?;
    if right.starts_with("no") {
        Ok(vec![])
    } else {
        let a = upto_last_space(left)?;
        right[..(right.len() - 1)]
            .split(", ")
            .map(|s| s.to_string())
            .map(|s: String| Ok((a.clone(), upto_last_space(&s)?[2..].to_string())))
            .collect::<Result<Vec<_>>>()
    }
}

fn weights(line: String) -> Result<HashMap<(String, String), usize>> {
    let v = line
        .split(" contain ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let left = v.first().context(InvalidLine {})?;
    let right = v.last().context(InvalidLine {})?;
    if right.starts_with("no") {
        Ok(HashMap::new())
    } else {
        let a = upto_last_space(left)?;
        right[..(right.len() - 1)]
            .split(", ")
            .map(|s| s.to_string())
            .map(|s: String| {
                Ok((
                    (a.clone(), upto_last_space(&s)?[2..].to_string()),
                    upto_last_space(&s)?[..1].parse().context(ParseInt {})?,
                ))
            })
            .collect::<Result<HashMap<_, _>>>()
    }
}

fn upto_last_space(s: &String) -> Result<String> {
    let last_space = s.rfind(" ").context(InvalidLine {})?;
    Ok(s[..last_space].to_string())
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

struct BiMap<S, T> {
    forward: HashMap<S, T>,
    reverse: HashMap<T, S>,
}

impl<S, T> BiMap<S, T>
where
    S: std::cmp::Eq,
    S: std::hash::Hash,
    T: std::cmp::Eq,
    T: std::hash::Hash,
    S: Copy,
    T: Copy,
{
    fn new<'b>() -> BiMap<S, T> {
        BiMap {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    fn insert(&mut self, s: S, t: T) {
        self.forward.insert(s, t);
        self.reverse.insert(t, s);
    }
    #[inline]
    fn get_forward<K>(&self, s: &K) -> Option<&T>
    where
        S: Borrow<K>,
        K: std::cmp::Eq + std::hash::Hash,
    {
        Some(self.forward.get(&s)?)
    }
    #[inline]
    fn get_reverse<K>(&self, t: &K) -> Option<&S>
    where
        T: Borrow<K>,
        K: std::cmp::Eq + std::hash::Hash,
    {
        Some(self.reverse.get(&t)?)
    }
}
