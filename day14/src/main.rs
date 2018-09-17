use std::collections::VecDeque;
use std::env;

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

    fn hash(&mut self, input: &[u8]) -> Vec<u8> {
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
            .collect()
    }
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    Free,
    Used,
    Region(u16),
}

fn part1(input: &str) {
    let count: u32 = (0..128)
        .map(|n| {
            let value = format!("{}-{}", input, n);
            let mut hasher = KnotHasher::default();
            let hash = hasher.hash(value.as_bytes());
            hash.iter().map(|b| b.count_ones()).sum::<u32>()
        })
        .sum();
    println!("part 1: {}", count);
}

fn part2(input: &str) {
    let mut grid: Vec<Vec<Cell>> = (0..128)
        .map(|n| {
            let value = format!("{}-{}", input, n);
            let mut hasher = KnotHasher::default();
            let hash = hasher.hash(value.as_bytes());
            hash.iter()
                .flat_map(|b| {
                    let binary = format!("{:08b}", b);
                    binary
                        .chars()
                        .map(|bit| match bit {
                            '0' => Cell::Free,
                            '1' => Cell::Used,
                            _ => unreachable!(),
                        })
                        .collect::<Vec<Cell>>()
                })
                .collect()
        })
        .collect();

    let mut current_region = 0;
    for cy in 0..128 {
        for cx in 0..128 {
            match grid[cy][cx] {
                Cell::Free | Cell::Region(_) => continue,
                Cell::Used => {}
            }

            current_region += 1;
            let mut queue = VecDeque::new();
            queue.push_back((cx, cy));

            while let Some((x, y)) = queue.pop_front() {
                if let Cell::Used = grid[y][x] {
                    grid[y][x] = Cell::Region(current_region);
                } else {
                    continue;
                }


                for &(dx, dy) in &[(0, -1), (-1, 0), (0, 1), (1, 0)] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || nx > 127 || ny < 0 || ny > 127 {
                        continue;
                    }

                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    println!("part 2: {}", current_region);
}

fn main() {
    let input = env::args().nth(1).expect("input");
    part1(&input);
    part2(&input);
}
