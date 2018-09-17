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
use std::env;
use std::fs::File;
use std::ops::{Add, Mul};
use std::io::{BufRead, BufReader};
use std::str;
use std::result;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl str::FromStr for Vector {
    type Err = Error;
    fn from_str(line: &str) -> Result<Vector> {
        let (_, vector_str) = line.split_at(2);
        let parts = vector_str
            .trim()
            .trim_left_matches('<')
            .trim_right_matches('>')
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<result::Result<Vec<i64>, _>>()?;
        let vector = Vector {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        };
        Ok(vector)
    }
}

impl Vector {
    fn distance(&self, other: Vector) -> i64 {
        (self.x + other.x).abs() + (self.y + other.y).abs() + (self.z + other.z).abs()
    }

    fn distance_from_origin(&self) -> i64 {
        self.distance(Vector::default())
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: i64) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl str::FromStr for Particle {
    type Err = Error;
    fn from_str(line: &str) -> Result<Particle> {
        let parts = line.split(", ").collect::<Vec<&str>>();
        let particle = Particle {
            position: parts[0].parse()?,
            velocity: parts[1].parse()?,
            acceleration: parts[2].parse()?,
        };
        Ok(particle)
    }
}

fn part1(iterations: usize, particles: &[Particle]) {
    let mut particles = particles.to_vec();

    for _ in 0..iterations {
        for particle in &mut particles {
            particle.velocity = particle.velocity + particle.acceleration;
        }

        for particle in &mut particles {
            particle.position = particle.position + particle.velocity;
        }
    }

    let (id, _) = particles
        .iter()
        .enumerate()
        .min_by_key(|&(_, particle)| particle.position.distance_from_origin())
        .unwrap();
    println!("part 1: {}", id);
}

fn part2(iterations: usize, particles: &[Particle]) {
    let mut particles = particles.to_vec();

    for _ in 0..iterations {
        for particle in &mut particles {
            particle.velocity = particle.velocity + particle.acceleration;
        }

        for particle in &mut particles {
            particle.position = particle.position + particle.velocity;
        }

        let mut colliding = particles
            .iter()
            .enumerate()
            .filter(|&(_, particle)| {
                particles
                    .iter()
                    .filter(|p| p.position == particle.position)
                    .count() > 1
            })
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        if !colliding.is_empty() {
            colliding.reverse();
            for i in colliding {
                particles.remove(i);
            }
        }
    }

    println!("part 2: {}", particles.len());
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let iterations = env::args().nth(2).expect("iterations").parse()?;
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let particles = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse())
        .collect::<Result<Vec<Particle>>>()?;

    part1(iterations, &particles);
    part2(iterations, &particles);

    Ok(())
}

quick_main!(run);
