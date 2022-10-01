use std::io::Result;

use crate::line::SourceLine;
use crate::parser::LineParser;

#[derive(Debug)]
pub struct Asm {
    pub comment: String,
    pub lines: Vec<String>,
}

impl Asm {
    pub fn new(source: &SourceLine, lines: Vec<String>) -> Self {
        Self {
            comment: format!("//{}", source.source),
            lines,
        }
    }

    pub fn unkown(comment: &str) -> Self {
        Self {
            comment: format!("//{}", comment),
            lines: vec!["//UNKOWN".to_string()],
        }
    }

    pub fn set_line(&mut self, line_num: usize, new_line: String) {
        self.lines[line_num] = new_line;
    }
}

impl Default for Asm {
    fn default() -> Self {
        Asm {
            comment: "//UNKOWN".to_string(),
            lines: vec!["//UNKOWN".to_string()],
        }
    }
}
pub struct AsmGen {
    // pub asm_dir: PathBuf,
    pub lbl_idx: i32,
    pub filename: String,
    pub asm_dir: String,
    asm_reader: AsmReader,
    // pub fn_lbl_stack: Vec<String>,
}

impl AsmGen {
    pub fn new(filename: &str) -> Result<Self> {
        // let cwd = env::current_dir().unwrap();
        // println!("{:?}", cwd);
        // Ok(Self { asm_dir: cwd })
        Ok(Self {
            filename: filename.to_string(),
            asm_dir: "src/asm".to_string(),
            lbl_idx: 0,
            asm_reader: AsmReader::new(),
        })
    }

    pub fn gen_init_asm(&mut self) -> Asm {
        let call_lines = self.asm_reader.call();
        let new_lbl_idx = &self.next_lbl_idx();

        let ret_addr = format!("{}$ret.{}", self.filename, new_lbl_idx);

        let mut asm = Asm {
            comment: "// Sys Init bootstrap".to_string(),
            lines: call_lines,
        };

        // set asm lines
        // write return address to stack
        asm.set_line(0, format!("@{ret_addr}"));

        // set arg offset
        asm.set_line(35, format!("@{}", "0"));
        // set function call address
        asm.set_line(47, format!("@{}", "Sys.init"));
        // write return label
        asm.set_line(49, format!("({ret_addr})"));

        // prepend init asm
        let init_lines = self.asm_reader.init();

        asm.lines = [init_lines, asm.lines].concat();

        asm
    }

    // ---
    // Public Asm factory methods
    // ---

    pub fn gen_ret_asm(&mut self, source: &SourceLine) -> Asm {
        let raw_lines = self.asm_reader.ret();

        Asm {
            comment: format!("//{}", source.source),
            lines: raw_lines,
        }
    }

    pub fn gen_func_asm(&mut self, source: &SourceLine) -> Asm {
        let raw_lines = self.asm_reader.func();

        let mut asm = Asm {
            comment: format!("//{}", source.source),
            lines: raw_lines,
        };

        // set lines
        asm.set_line(0, format!("({})", source.args.arg1));

        for n_local_args in 0..source.args.arg2.unwrap() {
            // get set local lcl to zero asm
            let mut set_lcl_asm = self.asm_reader.push_lcl();

            // set offset of LCL address
            set_lcl_asm[0] = format!("@{n_local_args}");

            for line in set_lcl_asm {
                // set first iteration of push local args
                asm.lines.push(line.to_string());
            }
        }

        asm
    }

    pub fn gen_call_asm(&mut self, source: &SourceLine) -> Asm {
        let raw_lines = self.asm_reader.call();
        let new_lbl_idx = &self.next_lbl_idx();

        let ret_addr = format!("{}$ret.{}", self.filename, new_lbl_idx);

        let mut asm = Asm {
            comment: format!("//{}", source.source),
            lines: raw_lines,
        };

        // set asm lines
        // write return address to stack
        asm.set_line(0, format!("@{ret_addr}"));
        // set function call address
        asm.set_line(47, format!("@{}", source.args.arg1));
        // write return label
        asm.set_line(49, format!("({ret_addr})"));

        // check if zero args passed to call, add space for return value on stack
        if source.args.arg2.unwrap() == 0 {
            // arg offset, number of args passed to funtion
            asm.set_line(35, "@1".to_string());
            let mut push_const_asm = self.asm_reader.push_const();
            push_const_asm[0] = "@0".to_string();
            push_const_asm.append(&mut asm.lines);
            asm.lines = push_const_asm;
        } else {
            // arg offset, number of args passed to funtion
            asm.set_line(35, format!("@{}", source.args.arg2.unwrap()));
        };

        asm
    }

    pub fn gen_if_asm(&mut self, source: &SourceLine) -> Asm {
        let lines = self.asm_reader.if_goto();

        let label = &source.args.arg1;

        let mut asm = Asm {
            comment: format!("//{}", source.source),
            lines,
        };
        asm.set_line(4, format!("@{label}"));
        asm
    }

    pub fn gen_goto_asm(&mut self, source: &SourceLine) -> Asm {
        let lines = vec![format!("@{}", source.args.arg1), "0;JMP".to_string()];

        Asm {
            comment: format!("//{}", source.source),
            lines,
        }
    }

    pub fn gen_label_asm(&mut self, source: &SourceLine) -> Asm {
        let lines = vec![format!("({})", source.args.arg1)];

        Asm {
            comment: format!("//{}", source.source),
            lines,
        }
    }

    // Arithmetic OP

    pub fn gen_add(&mut self, source: &SourceLine) -> Asm {
        self.gen_sum_asm(&source.source, "M=D+M")
    }

    pub fn gen_sub(&mut self, source: &SourceLine) -> Asm {
        self.gen_sum_asm(&source.source, "M=M-D")
    }

    pub fn gen_and(&mut self, source: &SourceLine) -> Asm {
        self.gen_sum_asm(&source.source, "M=D&M")
    }

    pub fn gen_or(&mut self, source: &SourceLine) -> Asm {
        self.gen_sum_asm(&source.source, "M=D|M")
    }

    // cmp template
    pub fn gen_eq(&mut self, source: &SourceLine) -> Asm {
        self.gen_cmp_asm(&source.source, "D;JEQ")
    }

    pub fn gen_lt(&mut self, source: &SourceLine) -> Asm {
        self.gen_cmp_asm(&source.source, "D;JLT")
    }

    pub fn gen_gt(&mut self, source: &SourceLine) -> Asm {
        self.gen_cmp_asm(&source.source, "D;JGT")
    }

    // negate template
    pub fn gen_neg(&mut self, source: &SourceLine) -> Asm {
        self.gen_neg_asm(&source.source, "M=-D")
    }

    pub fn gen_not(&mut self, source: &SourceLine) -> Asm {
        self.gen_neg_asm(&source.source, "M=!D")
    }

    // PUSH

    // push mem seg template
    pub fn gen_push(&mut self, source: &SourceLine, mem_seg_lbl: &str) -> Asm {
        // get index in mem seg to select

        let mem_seg_index = source.args.arg2.unwrap();

        // generate asm
        self.gen_push_mem_seg_asm(&source.source, mem_seg_index, mem_seg_lbl)
    }

    // push static_temp template
    pub fn gen_push_temp(&mut self, source: &SourceLine) -> Asm {
        let mem_seg_lbl = format!("R{}", source.args.arg2.unwrap() + 5);
        self.gen_push_static_temp_asm(&source.source, &mem_seg_lbl)
    }

    pub fn gen_push_static(&mut self, source: &SourceLine) -> Asm {
        let mem_seg_lbl = format!("{}.{}", self.filename, source.args.arg2.unwrap());
        self.gen_push_static_temp_asm(&source.source, &mem_seg_lbl)
    }

    // const template
    pub fn gen_push_const(&self, source: &SourceLine, value: i32) -> Asm {
        let lines = self.asm_reader.push_const();

        let mut asm = Asm {
            comment: format!("//{}", source.source),
            lines,
        };
        asm.set_line(0, format!("@{value}"));
        asm
    }

    pub fn gen_push_ptr(&self, source: &SourceLine) -> Asm {
        match source.args.arg2 {
            Some(0) => self.gen_push_ptr_asm(&source.source, "THIS"),
            Some(1) => self.gen_push_ptr_asm(&source.source, "THAT"),
            _ => Asm::unkown(&source.source),
        }
    }

    // POP

    pub fn gen_pop(&mut self, source: &SourceLine, mem_seg_lbl: &str) -> Asm {
        // get index in mem seg to select
        let mem_seg_index = source.args.arg2.unwrap();

        // generate asm
        self.gen_pop_mem_seg_asm(&source.source, mem_seg_index, mem_seg_lbl)
    }

    pub fn gen_pop_static(&mut self, source: &SourceLine) -> Asm {
        // get index in mem seg to select
        let mem_seg_lbl = format!("{}.{}", self.filename, source.args.arg2.unwrap());
        // generate asm
        self.gen_pop_static_temp_asm(&source.source, &mem_seg_lbl)
    }

    pub fn gen_pop_temp(&mut self, source: &SourceLine) -> Asm {
        // get index in mem seg to select
        let mem_seg_lbl = format!("R{}", source.args.arg2.unwrap() + 5);
        // generate asm
        self.gen_pop_static_temp_asm(&source.source, &mem_seg_lbl)
    }

    pub fn gen_pop_ptr(&self, source: &SourceLine) -> Asm {
        match source.args.arg2 {
            Some(0) => self.gen_pop_ptr_asm(&source.source, "THIS"),
            Some(1) => self.gen_pop_ptr_asm(&source.source, "THAT"),
            _ => Asm::unkown(&source.source),
        }
    }

    // ---
    // Private methods
    // ---

    // PUSH
    // ---

    fn gen_push_ptr_asm(&self, comment: &str, mem_seg_lbl: &str) -> Asm {
        let lines = self.asm_reader.push_ptr();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines,
        };

        asm.set_line(0, format!("@{mem_seg_lbl}"));
        asm
    }

    fn gen_push_static_temp_asm(&mut self, comment: &str, mem_seg_lbl: &str) -> Asm {
        let lines = self.asm_reader.push_static_temp();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines,
        };

        asm.set_line(0, format!("@{mem_seg_lbl}"));
        asm
    }

    fn gen_push_mem_seg_asm(&mut self, comment: &str, mem_index: i32, mem_seg_lbl: &str) -> Asm {
        let raw_lines = self.asm_reader.push_mem_seg();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        asm.set_line(0, format!("@{mem_index}"));
        asm.set_line(2, format!("@{mem_seg_lbl}"));

        asm
    }

    // POP
    // ---

    fn gen_pop_ptr_asm(&self, comment: &str, mem_seg_lbl: &str) -> Asm {
        let lines = self.asm_reader.pop_ptr();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines,
        };

        asm.set_line(4, format!("@{mem_seg_lbl}"));
        asm
    }

    fn gen_pop_mem_seg_asm(&mut self, comment: &str, mem_index: i32, mem_seg_lbl: &str) -> Asm {
        let raw_lines = self.asm_reader.pop_mem_seg();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        asm.set_line(6, format!("@{mem_index}"));
        asm.set_line(8, format!("@{mem_seg_lbl}"));

        asm
    }

    fn gen_pop_static_temp_asm(&mut self, comment: &str, mem_seg_lbl: &str) -> Asm {
        let raw_lines = self.asm_reader.pop_static_temp();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        asm.set_line(4, format!("@{mem_seg_lbl}"));
        asm
    }

    // ARITH
    // ---

    fn gen_neg_asm(&self, comment: &str, neg_cmd: &str) -> Asm {
        let raw_lines = self.asm_reader.neg();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        // set sum command
        asm.set_line(4, neg_cmd.to_string());
        asm
    }

    fn gen_sum_asm(&self, comment: &str, sum_cmd: &str) -> Asm {
        let raw_lines = self.asm_reader.sum();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        // set sum command
        asm.set_line(7, sum_cmd.to_string());
        asm
    }

    fn gen_cmp_asm(&mut self, comment: &str, compare_cmd: &str) -> Asm {
        let raw_lines = self.asm_reader.cmp();

        let mut asm = Asm {
            comment: format!("//{}", comment),
            lines: raw_lines,
        };

        // create labels for current asm set
        let true_lbl = format!("TRUE_{}", self.next_lbl_idx());
        let end_lbl = format!("END_{}", self.next_lbl_idx());

        // set labels in asm
        // true label
        asm.set_line(15, format!("({true_lbl})"));
        // end label
        asm.set_line(19, format!("({end_lbl})"));

        // set command to compare
        asm.set_line(9, compare_cmd.to_string());

        // set conditional jumps
        asm.set_line(8, format!("@{true_lbl}"));
        asm.set_line(13, format!("@{end_lbl}"));

        asm
    }

    fn next_lbl_idx(&mut self) -> i32 {
        self.lbl_idx += 1;
        self.lbl_idx
    }
}

pub struct AsmReader {
    pub call: Vec<String>,
    pub cmp: Vec<String>,
    pub func: Vec<String>,
    pub if_goto: Vec<String>,
    pub init: Vec<String>,
    pub neg: Vec<String>,
    pub pop_mem_seg: Vec<String>,
    pub pop_ptr: Vec<String>,
    pub pop_static_temp: Vec<String>,
    pub push_const: Vec<String>,
    pub push_lcl: Vec<String>,
    pub push_mem_seg: Vec<String>,
    pub push_ptr: Vec<String>,
    pub push_static_temp: Vec<String>,
    pub ret: Vec<String>,
    pub sum: Vec<String>,
}

impl AsmReader {
    pub fn new() -> Self {
        Self {
            call: Self::read_asm_source("call.asm"),
            cmp: Self::read_asm_source("cmp.asm"),
            func: Self::read_asm_source("func.asm"),
            if_goto: Self::read_asm_source("if_goto.asm"),
            init: Self::read_asm_source("init.asm"),
            neg: Self::read_asm_source("neg.asm"),
            pop_mem_seg: Self::read_asm_source("pop_mem_seg.asm"),
            pop_ptr: Self::read_asm_source("pop_ptr.asm"),
            pop_static_temp: Self::read_asm_source("pop_static_temp.asm"),
            push_const: Self::read_asm_source("push_const.asm"),
            push_lcl: Self::read_asm_source("push_lcl.asm"),
            push_mem_seg: Self::read_asm_source("push_mem_seg.asm"),
            push_ptr: Self::read_asm_source("push_ptr.asm"),
            push_static_temp: Self::read_asm_source("push_static_temp.asm"),
            ret: Self::read_asm_source("return.asm"),
            sum: Self::read_asm_source("sum.asm"),
        }
    }

    pub fn call(&self) -> Vec<String> {
        self.call.clone()
    }

    pub fn cmp(&self) -> Vec<String> {
        self.cmp.clone()
    }

    pub fn func(&self) -> Vec<String> {
        self.func.clone()
    }
    pub fn if_goto(&self) -> Vec<String> {
        self.if_goto.clone()
    }
    pub fn init(&self) -> Vec<String> {
        self.init.clone()
    }
    pub fn neg(&self) -> Vec<String> {
        self.neg.clone()
    }
    pub fn pop_mem_seg(&self) -> Vec<String> {
        self.pop_mem_seg.clone()
    }
    pub fn pop_ptr(&self) -> Vec<String> {
        self.pop_ptr.clone()
    }
    pub fn pop_static_temp(&self) -> Vec<String> {
        self.pop_static_temp.clone()
    }
    pub fn push_const(&self) -> Vec<String> {
        self.push_const.clone()
    }
    pub fn push_lcl(&self) -> Vec<String> {
        self.push_lcl.clone()
    }
    pub fn push_mem_seg(&self) -> Vec<String> {
        self.push_mem_seg.clone()
    }
    pub fn push_ptr(&self) -> Vec<String> {
        self.push_ptr.clone()
    }
    pub fn push_static_temp(&self) -> Vec<String> {
        self.push_static_temp.clone()
    }
    pub fn ret(&self) -> Vec<String> {
        self.ret.clone()
    }
    pub fn sum(&self) -> Vec<String> {
        self.sum.clone()
    }

    fn read_asm_source(filename: &str) -> Vec<String> {
        let asm_dir = "src/asm".to_string();
        let path = format!("{}/{}", asm_dir, filename);
        LineParser::parse_lines(&path).unwrap()
    }

    // fn get_filepath(&self, filename: &str) -> String {
    //     // let path = self.asm_dir.as_path();
    //     // let path = self.asm_dir.as_path();
    //     let asm_dir = &self.asm_dir;
    //     format!("{}/{}", asm_dir, filename)
    // }
}

#[cfg(test)]
mod test {
    // use super::*;

    #[test]
    fn test_asm_gen_add() {
        // let asm_gen = AsmGen::new("Filename").unwrap();
        // let asm = asm_gen.gen_arith_op("add numbers together", "add.asm");

        // assert_eq!(asm.lines.len(), 10);
        // assert_eq!(asm.comment, "add numbers together");
        // println!("{:?}", asm);
    }
}
