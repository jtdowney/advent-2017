#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;
use std::env;
use std::io::{self, Read};
use std::fs;

fn part1(input: &[u32]) {
    let mut sum = 0;
    let length = input.len();
    for i in 0..input.len() {
        let next = (i + 1) % length;
        if input[i] == input[next] {
            sum += input[i];
        }
    }

    println!("part 1: {}", sum);
}

fn part2(input: &[u32]) {
    let mut sum = 0;
    let length = input.len();
    let half = length / 2;
    for i in 0..input.len() {
        let next = (i + half) % length;
        if input[i] == input[next] {
            sum += input[i];
        }
    }

    println!("part 2: {}", sum);
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("missing filename");
    let file = fs::File::open(filename)?;
    let mut reader = io::BufReader::new(file);
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    let input: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    part1(&input);
    part2(&input);

    Ok(())
}

quick_main!(run);
