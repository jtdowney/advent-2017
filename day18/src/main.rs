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
use std::collections::{HashMap, VecDeque};
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
    Snd(Parameter),
    Set(Parameter, Parameter),
    Add(Parameter, Parameter),
    Mul(Parameter, Parameter),
    Mod(Parameter, Parameter),
    Rcv(Parameter),
    Jgz(Parameter, Parameter),
}

impl str::FromStr for Instruction {
    type Err = Error;
    fn from_str(line: &str) -> Result<Instruction> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let instruction = match parts[0] {
            "snd" => Instruction::Snd(parts[1].parse()?),
            "set" => Instruction::Set(parts[1].parse()?, parts[2].parse()?),
            "add" => Instruction::Add(parts[1].parse()?, parts[2].parse()?),
            "mul" => Instruction::Mul(parts[1].parse()?, parts[2].parse()?),
            "mod" => Instruction::Mod(parts[1].parse()?, parts[2].parse()?),
            "rcv" => Instruction::Rcv(parts[1].parse()?),
            "jgz" => Instruction::Jgz(parts[1].parse()?, parts[2].parse()?),
            i => unreachable!("unknown instruction: {}", i),
        };
        Ok(instruction)
    }
}

fn part1(instructions: &[Instruction]) {
    let mut registers = HashMap::new();
    let mut sound = 0;
    let mut ip = 0;
    loop {
        match instructions[ip] {
            Instruction::Snd(x) => {
                sound = x.value(&registers);
            }
            Instruction::Set(Parameter::Register(x), y) => {
                let value = y.value(&registers);
                registers.insert(x, value);
            }
            Instruction::Add(Parameter::Register(x), y) => {
                let rhs = y.value(&registers);
                let value = registers.entry(x).or_insert(0);
                *value += rhs;
            }
            Instruction::Mul(Parameter::Register(x), y) => {
                let rhs = y.value(&registers);
                let value = registers.entry(x).or_insert(0);
                *value *= rhs;
            }
            Instruction::Mod(Parameter::Register(x), y) => {
                let rhs = y.value(&registers);
                let value = registers.entry(x).or_insert(0);
                *value %= rhs;
            }
            Instruction::Rcv(x) => {
                if x.value(&registers) != 0 {
                    println!("part 1: {}", sound);
                    break;
                }
            }
            Instruction::Jgz(x, y) => {
                if x.value(&registers) > 0 {
                    ip = (ip as i64 + y.value(&registers)) as usize;
                    continue;
                }
            }
            i => unreachable!("invalid instruction: {:?}", i),
        }

        ip += 1;
    }
}

fn part2(instructions: &[Instruction]) {
    let mut blocked = [false; 2];
    let mut ip = [0usize; 2];

    let mut queue = vec![];
    let mut registers = vec![];
    for i in 0..2 {
        queue.push(VecDeque::new());
        registers.push(HashMap::new());
        registers[i].insert('p', i as i64);
    }

    let mut count = 0;
    let mut context = 0;
    loop {
        let other = (context + 1) % 2;
        match instructions[ip[context]] {
            Instruction::Snd(x) => {
                if context == 1 {
                    count += 1;
                }

                queue[other].push_back(x.value(&registers[context]));
            }
            Instruction::Set(Parameter::Register(x), y) => {
                let value = y.value(&registers[context]);
                registers[context].insert(x, value);
            }
            Instruction::Add(Parameter::Register(x), y) => {
                let rhs = y.value(&registers[context]);
                let value = registers[context].entry(x).or_insert(0);
                *value += rhs;
            }
            Instruction::Mul(Parameter::Register(x), y) => {
                let rhs = y.value(&registers[context]);
                let value = registers[context].entry(x).or_insert(0);
                *value *= rhs;
            }
            Instruction::Mod(Parameter::Register(x), y) => {
                let rhs = y.value(&registers[context]);
                let value = registers[context].entry(x).or_insert(0);
                *value %= rhs;
            }
            Instruction::Rcv(Parameter::Register(x)) => {
                match queue[context].pop_front() {
                    Some(v) => {
                        blocked[context] = false;
                        registers[context].insert(x, v);
                    }
                    None => {
                        if blocked[other] && queue[other].is_empty() {
                            break;
                        }

                        blocked[context] = true;
                        context = other;
                        continue;
                    }
                }
            }
            Instruction::Jgz(x, y) => {
                if x.value(&registers[context]) > 0 {
                    ip[context] = (ip[context] as i64 + y.value(&registers[context])) as usize;
                    continue;
                }
            }
            i => unreachable!("invalid instruction: {:?}", i),
        }

        ip[context] += 1;
    }

    println!("part 2: {}", count);
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
    part2(&instructions);

    Ok(())
}

quick_main!(run);
