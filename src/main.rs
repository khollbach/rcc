#![allow(unused_imports)] // for now

mod lexer;
mod parser;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read, Write};
use std::process::Command;
use std::{env, fs};

use anyhow::{Context, Result, ensure};
use regex::Regex;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    c_file: String,

    #[arg(long)]
    lex: bool,
    #[arg(long)]
    parse: bool,
    #[arg(long)]
    codegen: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    ensure!(
        args.c_file.ends_with(".c"),
        "input file name must end with `.c`"
    );
    let binary_file = &args.c_file[..args.c_file.len() - 2];
    let i_file = format!("{binary_file}.i");
    let s_file = format!("{binary_file}.s");

    let stage = Stage::from_args(&args)?;

    preprocess(&args.c_file, &i_file)?;
    compile(&i_file, &s_file, stage)?;
    if stage == Stage::CodeEmission {
        assemble(&s_file, &binary_file)?;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Lex,
    Parse,
    CodeGen,
    CodeEmission,
}

impl Stage {
    fn from_args(args: &Args) -> Result<Self> {
        let num_stage_flags = args.lex as u32 + args.parse as u32 + args.codegen as u32;
        ensure!(
            num_stage_flags <= 1,
            "expected at most 1 stage flag, got: lex={} parse={} codegen={}",
            args.lex,
            args.parse,
            args.codegen
        );
        Ok(if args.lex {
            Stage::Lex
        } else if args.parse {
            Stage::Parse
        } else if args.codegen {
            Stage::CodeGen
        } else {
            Stage::CodeEmission
        })
    }
}

fn preprocess(c_file: &str, i_file: &str) -> Result<()> {
    let success = Command::new("gcc")
        .args(["-E", "-P", c_file, "-o", i_file])
        .status()?
        .success();
    ensure!(success, "failed to preprocess");
    Ok(())
}

fn compile(i_file: &str, s_file: &str, stage: Stage) -> Result<()> {
    let mut file = File::open(&i_file)?;
    let mut c_code = String::new();
    file.read_to_string(&mut c_code)?;

    fs::remove_file(i_file)?;

    // seems reasonable!
    let tokens = lexer::tokenize(&c_code)?;
    dbg!(&tokens);

    if stage == Stage::Lex {
        return Ok(());
    }

    let ast = parser::parse_ast(&tokens)?;
    dbg!(ast);

    if stage == Stage::Parse {
        return Ok(());
    }

    let value = extract_return_value(&c_code)?;
    dbg!(value);

    if stage == Stage::CodeGen {
        return Ok(());
    }

    let asm = generate_assembly(value);
    print!("{asm}");

    debug_assert_eq!(stage, Stage::CodeGen);

    let mut out_file = File::create(s_file)?;
    out_file.write_all(asm.as_bytes())?;

    Ok(())
}

fn assemble(s_file: &str, binary_file: &str) -> Result<()> {
    let success = Command::new("gcc")
        .args([s_file, "-o", binary_file])
        .status()?
        .success();
    ensure!(success, "failed to assemble");
    Ok(())
}

/// Only works for programs of the form:
/// ```c
/// int main(){return 2;}
/// ```
fn extract_return_value(c_code: &str) -> Result<i32> {
    let re = r"^\s*int\s+main\s*\(\s*(void)?\s*\)\s*\{\s*return\s+(?<value>\d+)\s*;\s*\}\s*$";
    let re = Regex::new(re).unwrap();
    let caps = re.captures(c_code).context("regex")?;
    let value = caps["value"].parse()?;
    Ok(value)
}

/// Generate asm that returns a value (and doesn't do anything else).
fn generate_assembly(return_value: i32) -> String {
    format!(
        "\
    .globl main
main:
    movl ${return_value}, %eax
    ret
"
    )
}
