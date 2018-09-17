use std::env;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn part2(size: usize) -> usize {
    let mut direction = Direction::Right;
    let mut memory = vec![vec![1]];
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut max_x = 1;
    let mut max_y = 1;

    loop {
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

        memory[cur_y][cur_x] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ].iter()
            .filter_map(|&(dx, dy)| {
                memory.get((cur_y as isize + dy) as usize).and_then(|r| {
                    r.get((cur_x as isize + dx) as usize)
                })
            })
            .sum();

        if memory[cur_y][cur_x] > size {
            break;
        }
    }

    memory[cur_y][cur_x]
}

fn main() {
    let size = env::args().nth(1).expect("size of memory").parse().expect(
        "size to be a number",
    );
    let value = part2(size);
    println!("value: {}", value);
}
