#![allow(unused_imports)] // for now

use std::io::{self, Read, Write};
use std::env;
use std::fs::File;
use std::io::prelude::*;

use anyhow::{Context, Result};
use regex::Regex;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    in_file: String,

    #[arg(short, long)]
    out_file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut file = File::open(args.in_file)?;
    let mut c_code = String::new();
    file.read_to_string(&mut c_code)?;

    let value = extract_return_value(&c_code)?;
    let asm = generate_assembly(value);
    print!("{asm}");
    let mut out_file = File::create(args.out_file)?;
    out_file.write_all(asm.as_bytes())?;
    Ok(())
}

/// Only works for programs of the form:
/// ```c
/// int main(){return 2;}
/// ```
fn extract_return_value(c_code: &str) -> Result<i32> {
    let re = r"^\s*int\s+main\s*\(void\)\s*\{\s*return\s+(?<value>\d+)\s*;\s*\}\s*$";
    let re = Regex::new(re).unwrap();
    let caps = re.captures(c_code).context("regex")?;
    let value = caps["value"].parse()?;
    Ok(value)
}

/// Generate asm that returns a value (and doesn't do anything else).
fn generate_assembly(return_value: i32) -> String {
    format!("\
    .globl main
main:
    movl ${return_value}, %eax
    ret
")
}
