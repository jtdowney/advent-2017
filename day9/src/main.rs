use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug, Default)]
struct State {
    level: usize,
    in_garbage: bool,
    ignore_next: bool,
}

fn part1(input: &str) {
    let state = State::default();
    let (_, score) = input.chars().fold(
        (state, 0),
        |(state, score), item| match (item, state) {
            ('{',
             State {
                 in_garbage: false,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    level: state.level + 1,
                    ..state
                },
                score,
            ),
            ('}',
             State {
                 in_garbage: false,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    level: state.level - 1,
                    ..state
                },
                score + state.level,
            ),
            ('<',
             State {
                 in_garbage: false,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    in_garbage: true,
                    ..state
                },
                score,
            ),
            ('>',
             State {
                 in_garbage: true,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    in_garbage: false,
                    ..state
                },
                score,
            ),
            ('!', State { ignore_next: false, .. }) => (
                State {
                    ignore_next: true,
                    ..state
                },
                score,
            ),
            (_, State { ignore_next: true, .. }) => (
                State {
                    ignore_next: false,
                    ..state
                },
                score,
            ),
            _ => (state, score),
        },
    );

    println!("part 1: {}", score);
}

fn part2(input: &str) {
    let state = State::default();
    let (_, count) = input.chars().fold(
        (state, 0),
        |(state, count), item| match (item, state) {
            ('<',
             State {
                 in_garbage: false,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    in_garbage: true,
                    ..state
                },
                count,
            ),
            ('>',
             State {
                 in_garbage: true,
                 ignore_next: false,
                 ..
             }) => (
                State {
                    in_garbage: false,
                    ..state
                },
                count,
            ),
            ('!', State { ignore_next: false, .. }) => (
                State {
                    ignore_next: true,
                    ..state
                },
                count,
            ),
            (_, State { ignore_next: true, .. }) => (
                State {
                    ignore_next: false,
                    ..state
                },
                count,
            ),
            (_, State { in_garbage: true, .. }) => (state, count + 1),
            _ => (state, count),
        },
    );

    println!("part 2: {}", count);
}

fn main() {
    let filename = env::args().nth(1).expect("filename");
    let mut file = File::open(filename).expect("file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("input");

    part1(&input);
    part2(&input);
}
