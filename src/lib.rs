#![allow(unused_imports)] // for now

// data types
mod asm_ast;
mod ast;
mod token;

// compiler stages
mod code_emit;
mod code_gen;
mod lexer;
mod parser;

use crate::{asm_ast::AsmAst, ast::Ast, token::Token};
use anyhow::Result;

/// Compile a C program into x64 assembly.
pub fn compile(c_code: &str) -> Result<String> {
    let tokens = lex(c_code)?;
    let ast = parse(tokens)?;
    let asm_ast = code_gen(ast)?;
    let asm = code_emit(asm_ast)?;
    Ok(asm)
}

#[derive(Debug)]
pub struct LexerOutput {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct ParserOutput {
    ast: Ast,
}

#[derive(Debug)]
pub struct CodeGenOutput {
    asm_ast: AsmAst,
}

pub fn lex(c_code: &str) -> Result<LexerOutput> {
    let tokens = lexer::lex(c_code)?;
    Ok(LexerOutput { tokens })
}

pub fn parse(input: LexerOutput) -> Result<ParserOutput> {
    let ast = parser::parse(&input.tokens)?;
    Ok(ParserOutput { ast })
}

pub fn code_gen(input: ParserOutput) -> Result<CodeGenOutput> {
    let asm_ast = code_gen::code_gen(&input.ast)?;
    Ok(CodeGenOutput { asm_ast })
}

pub fn code_emit(input: CodeGenOutput) -> Result<String> {
    let asm = code_emit::code_emit(&input.asm_ast)?;
    Ok(asm)
}
