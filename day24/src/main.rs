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
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::result;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Component(usize, usize);

impl str::FromStr for Component {
    type Err = Error;
    fn from_str(line: &str) -> Result<Component> {
        let parts = line.trim().split('/').collect::<Vec<&str>>();
        let left = parts[0].parse()?;
        let right = parts[1].parse()?;
        Ok(Component(left, right))
    }
}

#[derive(Clone, Debug)]
struct Bridge {
    available_components: HashSet<Component>,
    pins: usize,
    strength: usize,
    length: usize,
}

impl Bridge {
    fn new(components: &HashSet<Component>) -> Bridge {
        Bridge {
            available_components: components.clone(),
            pins: 0,
            strength: 0,
            length: 0,
        }
    }

    fn is_compatiable(&self, component: Component) -> bool {
        self.pins == component.0 || self.pins == component.1
    }

    fn add_component(&self, component: Component) -> Bridge {
        let mut bridge = self.clone();
        bridge.available_components.remove(&component);
        bridge.strength += component.0 + component.1;
        bridge.length += 1;
        bridge.pins = if bridge.pins == component.0 {
            component.1
        } else {
            component.0
        };

        bridge
    }
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let components = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse())
        .collect::<Result<HashSet<Component>>>()?;

    let bridge = Bridge::new(&components);
    let mut candidates = vec![];
    let mut queue = components
        .iter()
        .filter(|&component| bridge.is_compatiable(*component))
        .cloned()
        .map(|component| bridge.add_component(component))
        .collect::<Vec<Bridge>>();
    while let Some(bridge) = queue.pop() {
        let components = bridge
            .available_components
            .iter()
            .filter(|&component| bridge.is_compatiable(*component))
            .cloned()
            .collect::<Vec<Component>>();
        if components.is_empty() {
            candidates.push(bridge);
            continue;
        }

        for component in components {
            let bridge = bridge.add_component(component);
            queue.push(bridge);
        }
    }

    let answer = candidates
        .iter()
        .max_by_key(|candidate| candidate.strength)
        .unwrap()
        .strength;
    println!("part 1: {}", answer);

    let max_length = candidates
        .iter()
        .max_by_key(|candidate| candidate.length)
        .unwrap()
        .length;
    let answer = candidates
        .iter()
        .filter(|candidate| candidate.length == max_length)
        .max_by_key(|candidate| candidate.strength)
        .unwrap()
        .strength;
    println!("part 2: {}", answer);

    Ok(())
}

quick_main!(run);
