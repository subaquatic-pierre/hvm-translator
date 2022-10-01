use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Args {
    pub arg1: String,
    pub arg2: Option<i32>,
}

#[derive(Debug)]
pub enum CommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
    UNKOWN,
}

#[derive(Debug)]
pub enum MemSeg {
    LCL,
    ARG,
    THIS,
    THAT,
    CONST,
    STATIC,
    PTR,
    TEMP,
    NONE,
}

#[derive(Debug)]
pub enum ArithOp {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
    NONE,
}

#[derive(Debug)]
pub struct SourceLine {
    pub source: String,
    pub args: Args,
    pub mem_seg: MemSeg,
    pub arith_op: ArithOp,
    pub cmd_type: CommandType,
}

impl SourceLine {
    pub fn new(
        source: &str,
        args: Args,
        mem_seg: MemSeg,
        arith_op: ArithOp,
        cmd_type: CommandType,
    ) -> Self {
        Self {
            source: source.to_string(),
            args,
            mem_seg,
            arith_op,
            cmd_type,
        }
    }
}

impl Display for SourceLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "source line -- {:#?} ", self)
    }
}
