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
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::result;

#[derive(Copy, Clone, Debug)]
struct Layer {
    depth: u16,
    range: u16,
    camera: u16,
    camera_reverse: bool,
}

impl Layer {
    fn step(&mut self) {
        if self.camera_reverse {
            self.camera -= 1;
        } else {
            self.camera += 1;
        }

        if self.camera == 0 {
            self.camera_reverse = false;
        } else if self.camera == self.range - 1 {
            self.camera_reverse = true;
        }
    }
}

impl str::FromStr for Layer {
    type Err = Error;
    fn from_str(line: &str) -> Result<Layer> {
        let parts: Vec<&str> = line.split(": ").collect();
        let depth = parts[0].parse()?;
        let range = parts[1].parse()?;

        Ok(Layer {
            camera: 0,
            camera_reverse: false,
            depth,
            range,
        })
    }
}

fn part1(input: &[Layer]) {
    let mut layers = input
        .iter()
        .map(|layer| (layer.depth, *layer))
        .collect::<HashMap<u16, Layer>>();
    let last_layer = *layers.keys().max().unwrap();
    let severity = (0..last_layer + 1).fold(0, |mut acc, t| {
        if layers.contains_key(&t) && layers[&t].camera == 0 {
            acc += layers[&t].depth * layers[&t].range;
        }

        for layer in layers.values_mut() {
            layer.step();
        }

        acc
    });

    println!("part 1: {}", severity);
}

struct StateCache {
    cache: HashMap<u32, State>,
}

impl StateCache {
    fn fetch(&mut self, time: u32) -> State {
        if self.cache.contains_key(&time) {
            self.cache[&time].clone()
        } else {
            let prev = self.fetch(time - 1);
            let next = prev.step();
            self.cache.insert(time, next.clone());
            next
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    layers: HashMap<u16, Layer>,
}

impl State {
    fn step(&self) -> State {
        let mut layers = self.layers.clone();
        for layer in layers.values_mut() {
            layer.step();
        }
        State { layers }
    }
}

fn part2(input: &[Layer]) {
    let layers = input
        .iter()
        .map(|layer| (layer.depth, *layer))
        .collect::<HashMap<u16, Layer>>();
    let last_layer = *(layers.keys().max().unwrap());
    let state = State { layers };
    let mut cache = StateCache {
        cache: {
            let mut cache = HashMap::new();
            cache.insert(0, state);
            cache
        },
    };

    for delay in 1.. {
        let caught = (0..last_layer + 1).any(|t| {
            let state = cache.fetch(delay + t as u32);
            state.layers.contains_key(&t) && state.layers[&t].camera == 0
        });

        if !caught {
            println!("part 2: {}", delay);
            break;
        }
    }
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse())
        .collect::<Result<Vec<Layer>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

quick_main!(run);
