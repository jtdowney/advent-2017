#![feature(slice_rotate)]

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
use std::io::Read;
use std::str;
use std::result;

#[derive(Copy, Clone, Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl str::FromStr for Move {
    type Err = Error;
    fn from_str(value: &str) -> Result<Move> {
        let mut ch = value.chars();
        let c = value.chars().nth(0).unwrap();
        let result = match c {
            's' => {
                let data = value.chars().skip(1).collect::<String>();
                let spin = data.parse()?;
                Move::Spin(spin)
            }
            'x' => {
                let data = value.chars().skip(1).collect::<String>();
                let mut parts = data.split('/');
                let a = parts.next().unwrap().parse()?;
                let b = parts.next().unwrap().parse()?;
                Move::Exchange(a, b)
            }
            'p' => {
                let data = value.chars().skip(1).collect::<String>();
                let mut parts = data.split('/');
                let a = parts.next().unwrap().chars().nth(0).unwrap();
                let b = parts.next().unwrap().chars().nth(0).unwrap();
                Move::Partner(a, b)
            }
            v => unreachable!("{:?}", v),
        };
        Ok(result)
    }
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let input = buffer
        .split(',')
        .map(str::trim)
        .map(str::parse)
        .collect::<Result<Vec<Move>>>()?;

    let mut dance = vec![
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
    ];

    for i in 0..1_000_000_000 {
        if i % 10_000_000 == 0 {
            println!("{}...", i);
        }

        for mv in &input {
            match *mv {
                Move::Spin(spin) => {
                    dance.rotate(16 - spin);
                }
                Move::Exchange(a, b) => {
                    dance.swap(a, b);
                }
                Move::Partner(a, b) => {
                    let a_pos = dance.iter().position(|&c| c == a).unwrap();
                    let b_pos = dance.iter().position(|&c| c == b).unwrap();
                    dance.swap(a_pos, b_pos);
                }
            }
        }
    }

    println!("part 1: {}", dance.iter().collect::<String>());

    Ok(())
}

quick_main!(run);
