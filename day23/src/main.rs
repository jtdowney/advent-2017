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
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::result;

#[derive(Copy, Clone, Debug)]
enum Parameter {
    Register(char),
    Value(i64),
}

impl Parameter {
    fn value(&self, registers: &HashMap<char, i64>) -> i64 {
        match *self {
            Parameter::Register(x) => *registers.get(&x).unwrap_or(&0),
            Parameter::Value(x) => x,
        }
    }
}

impl str::FromStr for Parameter {
    type Err = Error;
    fn from_str(line: &str) -> Result<Parameter> {
        let c = line.chars().nth(0).unwrap();
        if c.is_alphabetic() {
            Ok(Parameter::Register(c))
        } else {
            let value = line.parse()?;
            Ok(Parameter::Value(value))
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Set(Parameter, Parameter),
    Sub(Parameter, Parameter),
    Mul(Parameter, Parameter),
    Jnz(Parameter, Parameter),
}

impl str::FromStr for Instruction {
    type Err = Error;
    fn from_str(line: &str) -> Result<Instruction> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = match parts[0] {
            "set" => Instruction::Set(parts[1].parse()?, parts[2].parse()?),
            "sub" => Instruction::Sub(parts[1].parse()?, parts[2].parse()?),
            "mul" => Instruction::Mul(parts[1].parse()?, parts[2].parse()?),
            "jnz" => Instruction::Jnz(parts[1].parse()?, parts[2].parse()?),
            i => unreachable!("unknown instruction: {}", i),
        };
        Ok(instruction)
    }
}

fn part1(instructions: &[Instruction]) {
    let mut registers = HashMap::new();
    for c in b'a'..b'i' {
        registers.insert(c as char, 0);
    }

    let mut count = 0;
    let mut ip = 0;
    loop {
        match instructions.get(ip).cloned() {
            Some(Instruction::Set(Parameter::Register(x), y)) => {
                let value = y.value(&registers);
                registers.insert(x, value);
            }
            Some(Instruction::Sub(Parameter::Register(x), y)) => {
                let rhs = y.value(&registers);
                let value = registers.entry(x).or_insert(0);
                *value -= rhs;
            }
            Some(Instruction::Mul(Parameter::Register(x), y)) => {
                let rhs = y.value(&registers);
                let value = registers.entry(x).or_insert(0);
                *value *= rhs;
                count += 1;
            }
            Some(Instruction::Jnz(x, y)) => {
                if x.value(&registers) != 0 {
                    ip = (ip as i64 + y.value(&registers)) as usize;
                    continue;
                }
            }
            None => break,
            i => unreachable!("invalid instruction: {:?}", i),
        }

        ip += 1;
    }

    println!("part 1: {}", count);
}

fn part2() {
    let mut b = (84 * 100) + 100_000;
    let c = b + 17_000;
    let mut d;
    let mut f;
    let mut h = 0;

    while b <= c {
        f = 1;
        d = 2;

        while d != b {
            if b % d == 0 {
                f = 0;
                break;
            }

            d += 1;
        }

        if f == 0 {
            h += 1;
        }

        b += 17;
    }

    println!("part 2: {}", h);
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let instructions = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse())
        .collect::<Result<Vec<Instruction>>>()?;

    part1(&instructions);
    part2();

    Ok(())
}

quick_main!(run);
