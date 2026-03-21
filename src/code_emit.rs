use crate::asm_ast::{AsmAst, AsmProgram, Instruction, Operand, RoutineDef};

use anyhow::Result;

pub fn code_emit(asm_ast: &AsmAst) -> Result<String> {
    let mut asm = String::new();
    asm_ast.program.code_emit(&mut asm)?;
    Ok(asm)
}

impl AsmProgram {
    fn code_emit(&self, out: &mut String) -> Result<()> {
        match self {
            AsmProgram::RoutineDef(r) => {
                r.code_emit(out)?;
            }
        }
        out.push_str(".section .note.GNU-stack,\"\",@progbits\n");
        Ok(())
    }
}

const INDENT: &str = "    "; // 4 spaces

impl RoutineDef {
    fn code_emit(&self, out: &mut String) -> Result<()> {
        out.push_str(&format!("{INDENT}.globl {}\n", self.name));
        out.push_str(&format!("{}:\n", self.name));
        for instr in &self.instrs {
            instr.code_emit(out)?;
        }
        Ok(())
    }
}

impl Instruction {
    fn code_emit(&self, out: &mut String) -> Result<()> {
        match self {
            Instruction::Mov { src, dst } => {
                let src = src.code_emit();
                let dst = dst.code_emit();
                out.push_str(&format!("{INDENT}movl {}, {}\n", src, dst));
            }
            Instruction::Ret => {
                out.push_str(&format!("{INDENT}ret\n"));
            }
        }
        Ok(())
    }
}

impl Operand {
    fn code_emit(&self) -> String {
        match self {
            Operand::Immediate(value) => format!("${}", value),
            Operand::Register => "%eax".to_string(),
        }
    }
}
