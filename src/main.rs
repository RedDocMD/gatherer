use colored::*;
use error::Result as AssemblerResult;
use instruction::Instruction;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
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
        let mut parsed_asm = match parse_file(&args[1]) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e.to_string().red());
                process::exit(1);
            }
        };
        parsed_asm.assign_labels(0);
        if let Err(err) = parsed_asm.write_coe(&args[2]) {
            eprintln!("{}", err.to_string().red());
            process::exit(1);
        }
    }
}

struct ParsedAsm {
    instrs: Vec<Instruction>,
    labels: HashMap<String, usize>,
}

impl ParsedAsm {
    fn assign_labels(&mut self, off: u32) {
        for (idx, instr) in self.instrs.iter_mut().enumerate() {
            if instr.has_abs_label() {
                // Since our memory is small, we can
                // directly store the PC value in an AbsLabel
                let label_idx = self.labels[instr.get_label_name()] as u32;
                instr.set_abs_addr(off + 4 * label_idx);
            } else if instr.has_rel_label() {
                // It is relative to PC + 4
                let next_pc = 4 * (idx as u32 + 1);
                let label_idx = self.labels[instr.get_label_name()] as u32;
                let label_addr = 4 * label_idx;
                // Diff can be negative, so signed type
                let diff = label_addr as i32 - next_pc as i32;
                let imm = (diff as i16 >> 2) as u16;
                // But eventually, we want unsigned!
                instr.set_rel_addr(imm);
            }
        }
    }

    fn write_coe<P: AsRef<Path>>(&self, path: P) -> AssemblerResult<()> {
        let mut file = File::create(path)?;
        writeln!(&mut file, "memory_initialization_radix=2;")?;
        writeln!(&mut file, "memory_initialization_vector=")?;
        for (idx, instr) in self.instrs.iter().enumerate() {
            write!(&mut file, "{:032b}", instr.encode()?)?;
            if idx == self.instrs.len() - 1 {
                writeln!(&mut file, ";")?;
            } else {
                writeln!(&mut file, ",")?;
            }
        }
        Ok(())
    }
}

fn parse_file<P: AsRef<Path>>(filename: P) -> AssemblerResult<ParsedAsm> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut instrs = Vec::new();
    let mut labels = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let mut line = line.trim();
        // Ignore line comments
        if line.starts_with("//") {
            continue;
        }
        // Ignore end-of-line comments
        if let Some(slash_idx) = line.find("//") {
            line = &line[..slash_idx];
        }
        match detect_label(line) {
            Some(label) => {
                labels.insert(label, instrs.len());
            }
            None => instrs.extend(Instruction::from_str(line)?),
        }
    }
    Ok(ParsedAsm { instrs, labels })
}

fn detect_label(line: &str) -> Option<String> {
    lazy_static! {
        static ref LABEL_RE: Regex = Regex::new(r"([a-zA-Z0-9_]+):").unwrap();
    }
    LABEL_RE.captures(line).map(|caps| String::from(&caps[1]))
}
