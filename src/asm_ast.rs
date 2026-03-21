#[derive(Debug)]
pub struct AsmAst {
    pub program: AsmProgram,
}

#[derive(Debug)]
pub enum AsmProgram {
    RoutineDef(RoutineDef),
}

#[derive(Debug)]
pub struct RoutineDef {
    pub name: String,
    pub instrs: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dst: Operand },
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Immediate(i32),
    Register, // (currently just EAX)
}
