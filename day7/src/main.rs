#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
        foreign_links {
            ParseInt(::std::num::ParseIntError);
        }
    }
}

use errors::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;
use std::str;

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: u32,
    sum: Option<u32>,
    children: Vec<String>,
}

impl str::FromStr for Program {
    type Err = Error;
    fn from_str(line: &str) -> Result<Program> {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let weight = parts
            .next()
            .unwrap()
            .trim_matches(['(', ')'].as_ref())
            .parse()?;
        let children = match parts.next() {
            Some("->") => {
                parts
                    .map(|p| p.trim_matches(','))
                    .map(str::to_string)
                    .collect()
            }
            _ => vec![],
        };

        Ok(Program {
            name,
            weight,
            children,
            sum: None,
        })
    }
}

fn part1(input: &[Program]) -> String {
    let mut children = HashSet::new();

    for program in input {
        for child in &program.children {
            children.insert(child.clone());
        }
    }

    let parent = input
        .iter()
        .find(|program| !children.contains(&program.name))
        .unwrap();

    println!("part 1: {}", parent.name);

    parent.name.clone()
}

fn part2(input: &[Program], root: &str) {
    let mut towers: HashMap<String, Program> = input
        .iter()
        .map(|program| (program.name.clone(), program.clone()))
        .collect();

    let mut queue = VecDeque::new();
    queue.push_back(root.to_string());
    while let Some(name) = queue.pop_front() {
        let unresolved_children: Vec<String> = towers[&name]
            .children
            .iter()
            .filter(|&c| towers[c].sum.is_none())
            .map(|c| c.clone())
            .collect();
        if unresolved_children.is_empty() {
            let sum = towers[&name].weight +
                towers[&name]
                    .children
                    .iter()
                    .filter_map(|c| towers[&c.clone()].sum)
                    .sum::<u32>();
            towers.get_mut(&name).unwrap().sum = Some(sum);
        } else {
            for child in unresolved_children {
                queue.push_front(towers[&child].name.clone())
            }

            queue.push_back(name.to_string());
        }
    }

    println!("part 2:");
    let mut queue = VecDeque::new();
    queue.push_back(root.to_string());
    while let Some(name) = queue.pop_front() {
        let children_sums = towers[&name]
            .children
            .iter()
            .filter_map(|c| towers[&c.clone()].sum)
            .collect::<Vec<u32>>();
        if children_sums.windows(2).any(|parts| parts[0] != parts[1]) {
            println!("{}: {:?}", name, children_sums);

            for child in &towers[&name].children {
                println!(" -> {}: {:?}", child, towers[child]);
            }
        }

        for child in &towers[&name].children {
            queue.push_front(towers[child].name.clone())
        }
    }
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse())
        .collect::<Result<Vec<Program>>>()
        .expect("valid input");

    let root = part1(&input);
    part2(&input, &root);
}
