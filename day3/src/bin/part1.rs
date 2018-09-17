use std::env;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn part1(size: usize) -> i32 {
    let mut direction = Direction::Right;
    let mut memory = vec![vec![1]];
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut max_x = 1;
    let mut max_y = 1;
    let mut origin_x = 0;
    let mut origin_y = 0;

    for i in 2..size + 1 {
        match direction {
            Direction::Right => {
                cur_x += 1;

                if cur_x == max_x {
                    for row in memory.iter_mut() {
                        row.push(0);
                    }

                    max_x += 1;
                    direction = Direction::Up;
                }
            }
            Direction::Up => {
                if cur_y == 0 {
                    memory.insert(0, vec![0; max_x]);
                    origin_y += 1;
                    max_y += 1;
                    direction = Direction::Left;
                } else {
                    cur_y -= 1;
                }
            }
            Direction::Left => {
                if cur_x == 0 {
                    for row in memory.iter_mut() {
                        row.insert(0, 0);
                    }

                    origin_x += 1;
                    max_x += 1;
                    direction = Direction::Down;
                } else {
                    cur_x -= 1;
                }
            }
            Direction::Down => {
                cur_y += 1;

                if cur_y == max_y {
                    memory.push(vec![0; max_x]);
                    max_y += 1;
                    direction = Direction::Right;
                }
            }
        }

        memory[cur_y][cur_x] = i;
    }

    (cur_x as i32 - origin_x as i32).abs() + (cur_y as i32 - origin_y as i32).abs()
}

fn main() {
    let size = env::args().nth(1).expect("size of memory").parse().expect(
        "size to be a number",
    );
    let distance = part1(size);
    println!("distance: {}", distance);
}
