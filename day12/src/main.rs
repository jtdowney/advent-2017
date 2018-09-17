#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            ParseInt(::std::num::ParseIntError);
        }
    }
}

use errors::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;
use std::str;

fn parse_line(line: String) -> Result<(u16, Vec<u16>)> {
    let mut parts = line.split_whitespace();
    let origin = parts.next().unwrap().parse()?;
    let _ = parts.next();
    let connections = parts
        .map(|s| s.trim_matches(','))
        .map(str::parse)
        .collect::<result::Result<Vec<u16>, _>>()?;

    Ok((origin, connections))
}

fn part1(input: &HashMap<u16, Vec<u16>>) {
    let mut connected = HashSet::new();
    connected.insert(0);

    let mut last_size = 0;
    while last_size != connected.len() {
        last_size = connected.len();

        for (origin, connections) in input {
            if connected.contains(origin) {
                for connection in connections {
                    connected.insert(*connection);
                }
            }
        }
    }

    println!("part 1: {}", connected.len());
}

fn part2(input: &HashMap<u16, Vec<u16>>) {
    let mut connected: Vec<HashSet<u16>> = vec![];
    let mut need_search = false;
    loop {
        for origin in input.keys() {
            if !connected.iter().any(|set| set.contains(origin)) {
                connected.push(HashSet::new());
                connected.last_mut().unwrap().insert(*origin);
                need_search = true;
                break;
            }
        }

        if !need_search {
            break;
        } else {
            need_search = false;
        }

        let mut last_size = 0;
        while last_size != connected.last().unwrap().len() {
            last_size = connected.last().unwrap().len();

            for (origin, connections) in input {
                if connected.last().unwrap().contains(origin) {
                    for connection in connections {
                        connected.last_mut().unwrap().insert(*connection);
                    }
                }
            }
        }
    }

    println!("part 2: {}", connected.len());
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(parse_line)
        .collect::<Result<HashMap<u16, Vec<u16>>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

quick_main!(run);
