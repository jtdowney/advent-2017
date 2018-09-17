use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(mut input: Vec<i32>) {
    let mut ip: isize = 0;
    let mut count = 0;
    loop {
        if ip < 0 || ip >= input.len() as isize {
            break;
        }

        let prev = ip;
        ip = ip + input[ip as usize] as isize;
        input[prev as usize] += 1;

        count += 1;
    }

    println!("part 1: {}", count);
}

fn part2(mut input: Vec<i32>) {
    let mut ip: isize = 0;
    let mut count = 0;
    loop {
        if ip < 0 || ip >= input.len() as isize {
            break;
        }

        let prev = ip;
        let offset = input[ip as usize] as isize;
        ip = ip + offset;

        if offset >= 3 {
            input[prev as usize] -= 1;
        } else {
            input[prev as usize] += 1;
        }

        count += 1;
    }

    println!("part 1: {}", count);
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse())
        .collect::<Result<Vec<i32>, _>>()
        .expect("valid input");

    part1(input.clone());
    part2(input.clone());
}
