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
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;
use std::result;

fn run() -> Result<()> {
    let filename = env::args().nth(1).expect("filename");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let input = reader
        .lines()
        .filter_map(result::Result::ok)
        .collect::<Vec<String>>();

    Ok(())
}

quick_main!(run);
