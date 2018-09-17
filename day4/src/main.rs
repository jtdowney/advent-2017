extern crate itertools;

use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;
use std::str;

fn part1(input: &[Vec<String>]) {
    let count = input
        .iter()
        .filter(|phrase| {
            !phrase.iter().combinations(2).any(
                |words| words[0] == words[1],
            )
        })
        .count();
    println!("part 1: {}", count);
}

fn part2(input: &[Vec<String>]) {
    let count = input
        .iter()
        .filter(|phrase| {
            !phrase.iter().combinations(2).any(|words| {
                let mut a = words[0].chars().collect::<Vec<char>>();
                a.sort();
                let mut b = words[1].chars().collect::<Vec<char>>();
                b.sort();
                a == b
            })
        })
        .count();
    println!("part 2: {}", count);
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let reader = BufReader::new(file);
    let input: Vec<Vec<String>> = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.split_whitespace().map(str::to_string).collect())
        .collect();
    part1(&input);
    part2(&input);
}
