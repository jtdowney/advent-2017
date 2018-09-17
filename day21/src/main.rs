#[macro_use]
extern crate error_chain;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use errors::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str;
use std::result;

type Pixel = bool;

struct Rule {
    input: Pattern,
    output: Pattern,
}

impl str::FromStr for Rule {
    type Err = Error;
    fn from_str(line: &str) -> Result<Rule> {
        let mut patterns = line.trim()
            .split(" => ")
            .map(|part| part.parse())
            .collect::<Result<Vec<Pattern>>>()?;
        let rule = Rule {
            input: patterns.remove(0),
            output: patterns.remove(0),
        };

        Ok(rule)
    }
}

struct RuleBook {
    rules: HashMap<Pattern, Pattern>,
}

impl FromIterator<Rule> for RuleBook {
    fn from_iter<I: IntoIterator<Item = Rule>>(iter: I) -> Self {
        let rules = iter.into_iter()
            .map(|Rule {
                 ref input,
                 ref output,
             }| (input.clone(), output.clone()))
            .collect::<HashMap<Pattern, Pattern>>();

        RuleBook { rules }
    }
}

impl RuleBook {
    fn transform(&self, pattern: &Pattern) -> Pattern {
        let mut pattern = pattern.clone();
        for i in 0..8 {
            if self.rules.contains_key(&pattern) {
                return self.rules[&pattern].clone();
            }

            pattern = pattern.rotate();
            if i == 3 {
                pattern = pattern.flip();
            }
        }

        panic!("no match");
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pattern {
    pixels: Vec<Vec<Pixel>>,
}

impl Pattern {
    fn count_on(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&p| *p).count())
            .sum()
    }

    fn rotate(&self) -> Pattern {
        let size = self.pixels[0].len();
        let mut pixels = vec![];
        for i in 0..size {
            let mut row = vec![];
            for j in (0..size).rev() {
                row.push(self.pixels[j][i]);
            }
            pixels.push(row);
        }

        Pattern { pixels }
    }

    fn flip(&self) -> Pattern {
        let size = self.pixels[0].len();
        let start = 0;
        let end = size - 1;
        let mut pixels = vec![];
        for i in 0..size {
            let mut row = self.pixels[i].clone();
            row.swap(start, end);
            pixels.push(row);
        }

        Pattern { pixels }
    }
}

impl str::FromStr for Pattern {
    type Err = Error;
    fn from_str(line: &str) -> Result<Pattern> {
        let pixels = line.split('/')
            .map(|part| {
                part.chars().map(|c| c == '#').collect::<Vec<Pixel>>()
            })
            .collect::<Vec<Vec<Pixel>>>();
        let pattern = Pattern { pixels };

        Ok(pattern)
    }
}

#[derive(Clone)]
struct Image {
    pixels: Vec<Vec<Pixel>>,
}

impl Image {
    fn patterns(self) -> ImagePatterns {
        let image_side_size = self.pixels[0].len();
        let even_size = image_side_size % 2 == 0;
        let pattern_size = if even_size { 2 } else { 3 };
        let total_size = image_side_size / pattern_size;

        ImagePatterns {
            image: self,
            pattern_offset: 0,
            pattern_size,
            total_size,
        }
    }
}

struct ImagePatterns {
    image: Image,
    pattern_offset: usize,
    pattern_size: usize,
    total_size: usize,
}

impl Iterator for ImagePatterns {
    type Item = Pattern;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pattern_offset >= self.total_size * self.total_size {
            return None;
        }

        let startx = (self.pattern_offset % self.total_size) * self.pattern_size;
        let starty = (self.pattern_offset / self.total_size) * self.pattern_size;
        let pixels = (starty..starty + self.pattern_size)
            .map(|j| {
                self.image.pixels[j][startx..startx + self.pattern_size].to_vec()
            })
            .collect::<Vec<Vec<Pixel>>>();
        let pattern = Pattern { pixels };

        self.pattern_offset += 1;
        Some(pattern)
    }
}

impl FromIterator<Pattern> for Image {
    fn from_iter<I: IntoIterator<Item = Pattern>>(iter: I) -> Self {
        let patterns = iter.into_iter().collect::<Vec<Pattern>>();
        let size = (patterns.len() as f32).sqrt() as usize;
        let pattern_size = patterns[0].pixels.len();

        let mut pixels = vec![];
        for i in 0..size {
            let start = i * size;
            let end = (i + 1) * size;
            let row_patterns = &patterns[start..end];
            for j in 0..pattern_size {
                let row = row_patterns
                    .iter()
                    .flat_map(|pattern| pattern.pixels[j].clone())
                    .collect::<Vec<Pixel>>();
                pixels.push(row);
            }
        }

        Image { pixels }
    }
}

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let rule_book = reader
        .lines()
        .filter_map(result::Result::ok)
        .map(|line| line.parse::<Rule>())
        .collect::<Result<RuleBook>>()?;

    let starting_pattern = ".#./..#/###".parse()?;
    let mut image = vec![starting_pattern].into_iter().collect::<Image>();
    for i in 0..18 {
        if i == 5 {
            let count: usize = image
                .clone()
                .patterns()
                .map(|pattern| pattern.count_on())
                .sum();
            println!("part 1: {}", count);
        }

        image = image
            .patterns()
            .map(|pattern| rule_book.transform(&pattern))
            .collect();
    }

    let count: usize = image.patterns().map(|pattern| pattern.count_on()).sum();
    println!("part 2: {}", count);

    Ok(())
}

quick_main!(run);
