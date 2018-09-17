use std::env;
use std::result;

struct KnotHasher {
    current: usize,
    skip: usize,
    state: Vec<u8>,
}

impl Default for KnotHasher {
    fn default() -> KnotHasher {
        let mut state = Vec::with_capacity(256);
        for i in 0..256 {
            state.push(i as u8);
        }

        KnotHasher {
            current: 0,
            skip: 0,
            state,
        }
    }
}

impl KnotHasher {
    fn round(&mut self, input: &[u8]) {
        let length = self.state.len();
        for &i in input {
            let i = i as usize;
            for j in 0..i / 2 {
                let a = (self.current + j) % length;
                let b = (self.current + i - j - 1) % length;
                self.state.swap(a, b);
            }

            self.current = (self.current + i + self.skip) % length;
            self.skip += 1;
        }
    }

    fn hash(&mut self, input: &[u8]) -> String {
        let mut input = input.to_vec();
        input.push(17);
        input.push(31);
        input.push(73);
        input.push(47);
        input.push(23);

        for _ in 0..64 {
            self.round(&input);
        }

        self.state
            .chunks(16)
            .map(|block| block.iter().fold(0, |acc, b| acc ^ b))
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}

fn part1(input: &str) {
    let input: Vec<u8> = input
        .split(',')
        .map(|v| v.trim().parse())
        .filter_map(result::Result::ok)
        .collect();

    let mut hasher = KnotHasher::default();
    hasher.round(&input);

    let answer: u32 = hasher.state.iter().take(2).map(|&v| u32::from(v)).product();
    println!("part 1: {}", answer);
}

fn part2(input: &str) {
    let input = input.as_bytes();
    let mut hasher = KnotHasher::default();
    let hash = hasher.hash(input);
    println!("part 2: {}", hash);
}

fn main() {
    let input = env::args().nth(1).expect("input");

    part1(&input);
    part2(&input);
}
