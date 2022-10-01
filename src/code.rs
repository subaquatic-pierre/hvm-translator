use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

use crate::asm::{Asm, AsmGen};
use crate::line::ArithOp;
use crate::line::CommandType;
use crate::line::MemSeg;
use crate::line::SourceLine;

pub struct Code {
    asm_gen: AsmGen,
}

impl Code {
    pub fn new(filename: &str) -> Self {
        // let fn_slipt = filename.split('.').collect::<Vec<&str>>();
        // let filename = fn_slipt.get(0).unwrap().to_string();
        let asm_gen = AsmGen::new(filename).unwrap();

        Self { asm_gen }
    }

    pub fn gen_init_asm(&mut self) -> Asm {
        self.asm_gen.gen_init_asm()
    }

    // top level asm generator, calls private methods based on cmd type
    pub fn gen_asm(&mut self, source: &SourceLine) -> Asm {
        // get command type
        match source.cmd_type {
            CommandType::ARITHMETIC => self.gen_arith_asm(source),
            CommandType::PUSH => self.gen_push_asm(source),
            CommandType::POP => self.gen_pop_asm(source),
            CommandType::IF => self.asm_gen.gen_if_asm(source),
            CommandType::GOTO => self.asm_gen.gen_goto_asm(source),
            CommandType::LABEL => self.asm_gen.gen_label_asm(source),
            CommandType::CALL => self.asm_gen.gen_call_asm(source),
            CommandType::FUNCTION => self.asm_gen.gen_func_asm(source),
            CommandType::RETURN => self.asm_gen.gen_ret_asm(source),
            // unknown cmd_type
            _ => Asm::unkown(&source.source),
        }
    }

    fn gen_arith_asm(&mut self, source: &SourceLine) -> Asm {
        // match on arithmetic opertation
        match source.arith_op {
            ArithOp::ADD => self.asm_gen.gen_add(source),
            ArithOp::SUB => self.asm_gen.gen_sub(source),
            ArithOp::NEG => self.asm_gen.gen_neg(source),
            ArithOp::EQ => self.asm_gen.gen_eq(source),
            ArithOp::GT => self.asm_gen.gen_gt(source),
            ArithOp::LT => self.asm_gen.gen_lt(source),
            ArithOp::AND => self.asm_gen.gen_and(source),
            ArithOp::OR => self.asm_gen.gen_or(source),
            ArithOp::NOT => self.asm_gen.gen_not(source),
            // arithmetic op unknown
            _ => Asm::unkown(&source.source),
        }
    }

    fn gen_push_asm(&mut self, source: &SourceLine) -> Asm {
        match source.mem_seg {
            MemSeg::CONST => self
                .asm_gen
                .gen_push_const(source, source.args.arg2.unwrap()),
            MemSeg::LCL => self.asm_gen.gen_push(source, "LCL"),
            MemSeg::ARG => self.asm_gen.gen_push(source, "ARG"),
            MemSeg::THIS => self.asm_gen.gen_push(source, "THIS"),
            MemSeg::THAT => self.asm_gen.gen_push(source, "THAT"),
            MemSeg::PTR => self.asm_gen.gen_push_ptr(source),
            MemSeg::TEMP => self.asm_gen.gen_push_temp(source),
            MemSeg::STATIC => self.asm_gen.gen_push_static(source),
            // unknown mem seg
            _ => Asm::unkown(&source.source),
        }
    }

    pub fn gen_pop_asm(&mut self, source: &SourceLine) -> Asm {
        match source.mem_seg {
            MemSeg::LCL => self.asm_gen.gen_pop(source, "LCL"),
            MemSeg::ARG => self.asm_gen.gen_pop(source, "ARG"),
            MemSeg::THIS => self.asm_gen.gen_pop(source, "THIS"),
            MemSeg::THAT => self.asm_gen.gen_pop(source, "THAT"),
            MemSeg::PTR => self.asm_gen.gen_pop_ptr(source),
            MemSeg::TEMP => self.asm_gen.gen_pop_temp(source),
            MemSeg::STATIC => self.asm_gen.gen_pop_static(source),
            // unknown mem seg
            _ => Asm::unkown(&source.source),
        }
    }
}

pub struct CodeWriter {
    f: File,
}

impl CodeWriter {
    pub fn new(out_fn: &str) -> Result<Self> {
        let fn_split: Vec<&str> = out_fn.split('.').collect();
        let filename = format!("{}.asm", fn_split.get(0).unwrap());
        Ok(Self {
            f: File::create(filename)?,
        })
    }

    pub fn write_asm(&mut self, asm: &Asm) -> Result<()> {
        writeln!(self.f, "{}", asm.comment)?;

        if !asm.lines.is_empty() {
            for line in &asm.lines {
                writeln!(self.f, "{}", line)?;
            }
        }
        Ok(())
    }
}
