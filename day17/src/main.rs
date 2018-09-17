use std::env;

fn part1(cycle_size: usize) {
    let mut current_position = 0;
    let mut buffer = vec![0];
    for i in 1..2018 {
        current_position = ((current_position + cycle_size) % buffer.len()) + 1;
        buffer.insert(current_position, i);
    }

    println!("part 1: {}", buffer[current_position + 1]);
}

fn part2(cycle_size: usize) {
    let mut current_position = 0;
    let mut value = 0;
    for i in 1..50_000_001 {
        current_position = ((current_position + cycle_size) % i) + 1;
        if current_position == 1 {
            value = i;
        }
    }

    println!("part 2: {}", value);
}

fn main() {
    let cycle_size = env::args()
        .nth(1)
        .unwrap_or_else(|| "3".to_string())
        .parse()
        .expect("valid input");

    part1(cycle_size);
    part2(cycle_size);
}
