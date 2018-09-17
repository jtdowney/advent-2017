#[macro_use]
extern crate error_chain;
extern crate regex;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Regex(::regex::Error);
            ParseChar(::std::char::ParseCharError);
            ParseInt(::std::num::ParseIntError);
        }
    }
}

use errors::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::str;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl str::FromStr for Direction {
    type Err = Error;
    fn from_str(direction: &str) -> Result<Direction> {
        let result = match direction {
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => unreachable!(),
        };
        Ok(result)
    }
}

#[derive(Clone, Copy, Debug)]
struct Rule {
    write_value: u8,
    next_direction: Direction,
    next_state: char,
}

impl<'a> From<regex::Captures<'a>> for Rule {
    fn from(capture: regex::Captures) -> Rule {
        Rule {
            write_value: capture
                .name("write_value")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            next_direction: capture
                .name("next_direction")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            next_state: capture
                .name("next_state")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
        }
    }
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let mut file = File::open(filename)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let rule_regex = Regex::new(
        r"  If the current value is (?P<value>\d):
    - Write the value (?P<write_value>\d).
    - Move one slot to the (?P<next_direction>left|right).
    - Continue with state (?P<next_state>\w)."
    )?;
    let state_regex = Regex::new(r"In state (?P<state>\w):(?:.*\n){9}")?;
    let start_regex = Regex::new(r"Begin in state (?P<state>\w).\nPerform a diagnostic checksum after (?P<steps>\d+) steps.")?;
    let start_capture = start_regex.captures(&input).unwrap();

    let rules = state_regex
        .captures_iter(&input)
        .flat_map(|state_capture| {
            let state = state_capture.name("state").unwrap().as_str();
            rule_regex
                .captures_iter(&state_capture[0])
                .map(|capture| {
                    let value = capture.name("value").unwrap().as_str();
                    let key = format!("{}-{}", state, value);
                    let rule = Rule::from(capture);
                    (key, rule)
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    let steps = start_capture.name("steps").unwrap().as_str().parse()?;
    let mut current_state: char = start_capture.name("state").unwrap().as_str().parse()?;
    let mut current_direction;
    let mut current_position = 0;
    let mut tape = HashMap::new();

    for _ in 0..steps {
        let value = tape.entry(current_position).or_insert(0);
        let rule = rules[&format!("{}-{}", current_state, value)];
        *value = rule.write_value;
        current_direction = rule.next_direction;
        current_state = rule.next_state;

        match current_direction {
            Direction::Left => current_position -= 1,
            Direction::Right => current_position += 1,
        }
    }

    let answer = tape.values().filter(|&value| *value == 1).count();
    println!("answer: {}", answer);

    Ok(())
}

quick_main!(run);
