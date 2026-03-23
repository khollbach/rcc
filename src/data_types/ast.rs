use anyhow::{Context, Result, bail, ensure};

#[derive(Debug)]
pub struct Ast {
    pub program: Program,
}

#[derive(Debug)]
pub enum Program {
    FunctionDef(FunctionDef),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub name: String,
    pub body: Statement,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Constant(i32),
}
