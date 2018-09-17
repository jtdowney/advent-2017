use std::env;
use std::i32;

const FACTOR_A: u32 = 16_807;
const FACTOR_B: u32 = 48_271;

struct Generator {
    previous: u32,
    factor: u32,
}

impl Iterator for Generator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let previous = u64::from(self.previous);
        let factor = u64::from(self.factor);
        self.previous = ((previous * factor) % i32::MAX as u64) as u32;
        Some(self.previous)
    }
}

fn part1(seed_a: u32, seed_b: u32) {
    let generator_a = Generator {
        previous: seed_a,
        factor: FACTOR_A,
    };
    let generator_b = Generator {
        previous: seed_b,
        factor: FACTOR_B,
    };

    let count = generator_a
        .zip(generator_b)
        .take(40_000_000)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count();

    println!("part 1: {}", count);
}

fn part2(seed_a: u32, seed_b: u32) {
    let generator_a = Generator {
        previous: seed_a,
        factor: FACTOR_A,
    };
    let generator_b = Generator {
        previous: seed_b,
        factor: FACTOR_B,
    };

    let count = generator_a
        .filter(|a| a % 4 == 0)
        .zip(generator_b.filter(|b| b % 8 == 0))
        .take(5_000_000)
        .filter(|&(a, b)| a as u16 == b as u16)
        .count();

    println!("part 2: {}", count);
}

fn main() {
    let seed_a = env::args()
        .nth(1)
        .unwrap_or_else(|| "65".to_string())
        .parse()
        .expect("seed for a");
    let seed_b = env::args()
        .nth(2)
        .unwrap_or_else(|| "8921".to_string())
        .parse()
        .expect("seed for b");

    part1(seed_a, seed_b);
    part2(seed_a, seed_b);
}
