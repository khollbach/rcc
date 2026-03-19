#![allow(unused_imports)] // for now

use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut c_code = String::new();
    io::stdin().lock().read_to_string(&mut c_code)?;

    let value = extract_return_value(&c_code)?;
    let asm = generate_assembly(value);
    print!("{asm}");

    Ok(())
}

/// Only works for programs of the form:
/// ```c
/// int main(){return 2;}
/// ```
fn extract_return_value(c_code: &str) -> Result<i32> {
    let re = r"^\s*int\s+main\s*\(\s*\)\s*\{\s*return\s+(?<value>\d+)\s*;\s*\}\s*$";
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
