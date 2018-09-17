#[macro_use]
extern crate error_chain;
extern crate itertools;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;
use itertools::Itertools;
use std::env;
use std::io::{self, BufRead};
use std::fs;
use std::result;
use std::str;

fn part1(input: &[Vec<u32>]) {
    let sum: u32 = input
        .iter()
        .map(|row| {
            let (min, max) = row.iter().fold(
                (<u32>::max_value(), <u32>::min_value()),
                |(mut min, mut max), &value| {
                    if value > max {
                        max = value;
                    }

                    if value < min {
                        min = value;
                    }

                    (min, max)
                },
            );

            max - min
        })
        .sum();

    println!("part 1: {}", sum);
}

fn part2(input: &[Vec<u32>]) {
    let sum: u32 = input
        .iter()
        .map(|row| {
            row.iter().combinations(2).fold(
                0,
                |acc, items| if items[0] % items[1] ==
                    0
                {
                    items[0] / items[1]
                } else if items[1] % items[0] == 0 {
                    items[1] / items[0]
                } else {
                    acc
                },
            )
        })
        .sum();

    println!("part 2: {}", sum);
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("missing filename");
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let input: Vec<Vec<u32>> = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .filter_map(result::Result::ok)
                .collect()
        })
        .collect();

    part1(&input);
    part2(&input);

    Ok(())
}

quick_main!(run);
