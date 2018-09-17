use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Position(isize, isize);

impl Position {
    fn neighbors(&self) -> Vec<Position> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .map(|&(dx, dy)| (self.0 + dx, self.1 + dy))
            .map(|(x, y)| Position(x, y))
            .collect()
    }

    fn next(&self, direction: Direction) -> Position {
        match direction {
            Direction::Down => Position(self.0, self.1 + 1),
            Direction::Up => Position(self.0, self.1 - 1),
            Direction::Left => Position(self.0 - 1, self.1),
            Direction::Right => Position(self.0 + 1, self.1),
        }
    }

    fn direction(&self, other: Position) -> Direction {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        match (dx, dy) {
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            _ => unreachable!("direction: {:?} => {:?}", self, other),
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<u8>>);

impl Map {
    fn get(&self, position: Position) -> Option<char> {
        self.0
            .get(position.1 as usize)
            .and_then(|row| row.get(position.0 as usize))
            .map(|b| *b as char)
    }
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let start = input[0].iter().position(|&b| b == b'|').unwrap();
    let map = Map(input);

    let mut steps = 0;
    let mut current_position = Position(start as isize, 0);
    let mut current_direction = Direction::Down;
    let mut letters_seen = vec![];
    loop {
        steps += 1;

        let next_position = current_position.next(current_direction);
        match map.get(next_position) {
            Some('+') => {
                let forward_position = next_position.next(current_direction);
                match map.get(forward_position) {
                    Some(' ') | None => {
                        let neighbors = next_position.neighbors();
                        let neighbor = neighbors
                            .iter()
                            .filter(|&neighbor| *neighbor != current_position)
                            .filter(|&neighbor| {
                                *neighbor != next_position.next(current_direction)
                            })
                            .find(|&&position| map.get(position) != Some(' '));
                        current_direction = neighbor
                            .expect("unable to locate a neighbor")
                            .direction(next_position);
                    }
                    _ => {}
                }
            }
            Some(c) if c.is_alphabetic() => {
                letters_seen.push(c);
            }
            None | Some(' ') => break,
            Some(_) => {}
        }

        current_position = next_position;
    }

    println!("part 1: {}", letters_seen.iter().collect::<String>());
    println!("part 2: {}", steps);
}
