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
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::str;

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl str::FromStr for Direction {
    type Err = Error;
    fn from_str(value: &str) -> Result<Direction> {
        let direction = match value {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "nw" => Direction::NorthWest,
            "s" => Direction::South,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            d => bail!("unknown direction: {}", d),
        };

        Ok(direction)
    }
}

impl Direction {
    fn movement(&self) -> Hex {
        match *self {
            Direction::North => Hex(1, 0, -1),
            Direction::NorthEast => Hex(1, -1, 0),
            Direction::NorthWest => Hex(0, 1, -1),
            Direction::South => Hex(-1, 0, 1),
            Direction::SouthEast => Hex(0, -1, 1),
            Direction::SouthWest => Hex(-1, 1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Hex(i32, i32, i32);

impl Add for Hex {
    type Output = Hex;
    fn add(self, other: Hex) -> Hex {
        Hex(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Hex {
    fn distance(&self, other: Hex) -> i32 {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()) / 2
    }
}

fn part1(input: &[Direction]) {
    let start = Hex(0, 0, 0);
    let end = input.iter().fold(start, |acc, direction| {
        let movement = direction.movement();
        acc + movement
    });

    let distance = start.distance(end);
    println!("part 1: {}", distance);
}

fn part2(input: &[Direction]) {
    let start = Hex(0, 0, 0);
    let (max, _) = input.iter().fold((0, start), |(max, acc), direction| {
        let movement = direction.movement();
        let current = acc + movement;
        let distance = start.distance(current);
        let max = if distance > max { distance } else { max };

        (max, current)
    });

    println!("part 2: {}", max);
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let input = input
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<Vec<Direction>>>()?;

    part1(&input);
    part2(&input);

    Ok(())
}

quick_main!(run);
