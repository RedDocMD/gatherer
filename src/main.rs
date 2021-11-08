use colored::*;
use error::Result as AssemblerResult;
use instruction::Instruction;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process,
};

mod error;
mod instruction;

#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "{}",
            format!("Usage: {} <input-file> <output-file>", args[0]).red()
        );
        process::exit(1);
    } else {
        let parsed_asm = match parse_file(&args[1]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                process::exit(1);
            }
        };
    }
}

struct ParsedAsm {
    instrs: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl ParsedAsm {
    fn assign_labels(&mut self) {}
}

fn parse_file<P: AsRef<Path>>(filename: P) -> AssemblerResult<ParsedAsm> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut instrs = Vec::new();
    let mut labels = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        match detect_label(line) {
            Some(label) => {
                labels.insert(String::from(label), instrs.len());
            }
            None => instrs.push(Instruction::try_from(line)?),
        }
    }
    Ok(ParsedAsm { instrs, labels })
}

fn detect_label(line: &str) -> Option<String> {
    lazy_static! {
        static ref LABEL_RE: Regex = Regex::new(r"([a-zA-Z0-9_]+):").unwrap();
    }
    match LABEL_RE.captures(line) {
        Some(caps) => Some(String::from(&caps[1])),
        None => None,
    }
}
