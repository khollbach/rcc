use crate::{
    asm_ast::{AsmAst, AsmProgram, Instruction, Operand, RoutineDef},
    ast::{Ast, Expr, FunctionDef, Program, Statement},
};

use anyhow::Result;

pub fn code_gen(ast: &Ast) -> Result<AsmAst> {
    ast.program.code_gen()
}

impl Program {
    pub fn code_gen(&self) -> Result<AsmAst> {
        match self {
            Program::FunctionDef(f) => {
                let routine = f.code_gen()?;
                let program = AsmProgram::RoutineDef(routine);
                Ok(AsmAst { program })
            }
        }
    }
}

impl FunctionDef {
    pub fn code_gen(&self) -> Result<RoutineDef> {
        Ok(RoutineDef {
            name: self.name.clone(),
            instrs: self.body.code_gen()?,
        })
    }
}

impl Statement {
    pub fn code_gen(&self) -> Result<Vec<Instruction>> {
        let operand = match self {
            Statement::Return(expr) => expr.code_gen()?,
        };
        Ok(vec![
            Instruction::Mov {
                src: operand,
                dst: Operand::Register,
            },
            Instruction::Ret,
        ])
    }
}

impl Expr {
    pub fn code_gen(&self) -> Result<Operand> {
        match self {
            Expr::Constant(value) => Ok(Operand::Immediate(*value)),
        }
    }
}
