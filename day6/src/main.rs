use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, BufReader};
use std::str;

fn solve(mut input: Vec<u32>) {
    let length = input.len();
    let mut count = 0;
    let mut history = HashMap::new();
    history.insert(input.clone(), count);

    loop {
        count += 1;

        let (largest, size) = input.iter().enumerate().fold(
            (0, 0),
            |acc, (i, &v)| if v > acc.1 {
                (i, v)
            } else {
                acc
            },
        );

        input[largest] = 0;
        for i in 1..size + 1 {
            let position = (largest + i as usize) % length;
            input[position] += 1;
        }

        if history.contains_key(&input) {
            let answer = count - history[&input];
            println!("part 2: {}", answer);
            break;
        } else {
            history.insert(input.clone(), count);
        }
    }

    println!("part 1: {}", count);
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename).expect("file");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("valid input");

    let input = buffer
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()
        .expect("valid input");

    solve(input);
}
