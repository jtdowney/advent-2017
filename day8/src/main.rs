#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{
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
use std::result;
use std::str;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Increase,
    Decrease,
}

impl str::FromStr for Operation {
    type Err = Error;
    fn from_str(op: &str) -> Result<Operation> {
        let result = match op {
            "inc" => Operation::Increase,
            "dec" => Operation::Decrease,
            _ => bail!("invalid operation {}", op),
        };

        Ok(result)
    }
}

#[derive(Debug, Copy, Clone)]
enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl str::FromStr for Comparison {
    type Err = Error;
    fn from_str(op: &str) -> Result<Comparison> {
        let result = match op {
            "==" => Comparison::Equal,
            ">" => Comparison::GreaterThan,
            ">=" => Comparison::GreaterThanOrEqual,
            "<" => Comparison::LessThan,
            "<=" => Comparison::LessThanOrEqual,
            "!=" => Comparison::NotEqual,
            _ => bail!("invalid comparison {}", op),
        };

        Ok(result)
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    register: String,
    operation: Operation,
    value: i32,
    comparison_register: String,
    comparison: Comparison,
    comparison_value: i32,
}

impl str::FromStr for Instruction {
    type Err = Error;
    fn from_str(line: &str) -> Result<Instruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let register = parts[0].to_string();
        let operation = parts[1].parse()?;
        let value = parts[2].parse()?;

        if parts[3] != "if" {
            bail!("invalid instruction");
        }

        let comparison_register = parts[4].to_string();
        let comparison = parts[5].parse()?;
        let comparison_value = parts[6].parse()?;

        Ok(Instruction {
            register,
            operation,
            value,
            comparison_register,
            comparison,
            comparison_value,
        })
    }
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

    let mut environment = HashMap::new();
    let mut max = 0;
    for instruction in instructions {
        let lhs = *environment
            .get(&instruction.comparison_register)
            .unwrap_or(&0);
        let rhs = instruction.comparison_value;
        let result = match instruction.comparison {
            Comparison::Equal => lhs == rhs,
            Comparison::GreaterThan => lhs > rhs,
            Comparison::GreaterThanOrEqual => lhs >= rhs,
            Comparison::LessThan => lhs < rhs,
            Comparison::LessThanOrEqual => lhs <= rhs,
            Comparison::NotEqual => lhs != rhs,
        };

        if result {
            let value = *environment.get(&instruction.register).unwrap_or(&0);
            let value = match instruction.operation {
                Operation::Increase => value + instruction.value,
                Operation::Decrease => value - instruction.value,
            };

            if value > max {
                max = value;
            }

            environment.insert(instruction.register.clone(), value);
        }
    }

    let (key, value) = environment.iter().max_by_key(|&(_, &v)| v).unwrap();
    println!("part 1: {}={}", key, value);
    println!("part 2: {}", max);

    Ok(())
}

quick_main!(run);
