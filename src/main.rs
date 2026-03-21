#![allow(unused_imports)] // for now

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

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
    let tokens = rcc::lex(&c_code)?;
    dbg!(&tokens);

    if stage == Stage::Lex {
        return Ok(());
    }

    let ast = rcc::parse(tokens)?;
    dbg!(&ast);

    if stage == Stage::Parse {
        return Ok(());
    }

    let asm_ast = rcc::code_gen(ast)?;
    dbg!(&asm_ast);

    if stage == Stage::CodeGen {
        return Ok(());
    }

    let asm = rcc::code_emit(asm_ast)?;
    print!("{}", asm);

    debug_assert_eq!(stage, Stage::CodeEmission);

    let mut file = File::create(s_file)?;
    file.write_all(asm.as_bytes())?;

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
