#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(isize, isize);

impl Point {
    fn next(&self, direction: Direction) -> Point {
        match direction {
            Direction::Down => Point(self.0, self.1 + 1),
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Up => Point(self.0, self.1 - 1),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Status {
    Clean,
    Flagged,
    Infected,
    Weakened,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn turn_around(&self) -> Direction {
        self.turn_left().turn_left()
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
        }
    }
}

fn part1(grid: &mut HashMap<Point, Status>) {
    let mut current_position = Point(0, 0);
    let mut current_direction = Direction::Up;
    let mut infections = 0;
    for _ in 0..10_000 {
        let current_status = *grid.get(&current_position).unwrap_or(&Status::Clean);
        match current_status {
            Status::Clean => {
                grid.insert(current_position, Status::Infected);
                current_direction = current_direction.turn_left();
                infections += 1;
            }
            Status::Infected => {
                grid.insert(current_position, Status::Clean);
                current_direction = current_direction.turn_right();
            }
            _ => unreachable!(),
        }

        current_position = current_position.next(current_direction);
    }

    println!("part 1: {}", infections);
}

fn part2(grid: &mut HashMap<Point, Status>) {
    let mut current_position = Point(0, 0);
    let mut current_direction = Direction::Up;
    let mut infections = 0;
    for _ in 0..10_000_000 {
        let current_status = *grid.get(&current_position).unwrap_or(&Status::Clean);
        match current_status {
            Status::Clean => {
                grid.insert(current_position, Status::Weakened);
                current_direction = current_direction.turn_left();
            }
            Status::Flagged => {
                grid.insert(current_position, Status::Clean);
                current_direction = current_direction.turn_around();
            }
            Status::Infected => {
                grid.insert(current_position, Status::Flagged);
                current_direction = current_direction.turn_right();
            }
            Status::Weakened => {
                grid.insert(current_position, Status::Infected);
                infections += 1;
            }
        }

        current_position = current_position.next(current_direction);
    }

    println!("part 2: {}", infections);
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let reader = BufReader::new(file);
    let grid = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let offset = (grid.len() / 2) as isize;
    let points = iproduct!(-offset..offset + 1, -offset..offset + 1);
    let input = points
        .filter(|&(x, y)| {
            let offsetx = (x + offset) as usize;
            let offsety = (y + offset) as usize;
            grid[offsety][offsetx] == b'#'
        })
        .map(|(x, y)| (Point(x, y), Status::Infected))
        .collect::<HashMap<_, _>>();

    part1(&mut input.clone());
    part2(&mut input.clone());
}
