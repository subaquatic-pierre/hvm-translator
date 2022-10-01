use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use crate::line::{Args, ArithOp, CommandType, MemSeg, SourceLine};

pub struct Parser {
    in_fn: String, // pub lines: Vec<SourceLine>
}

impl Parser {
    pub fn new(in_fn: &str) -> Self {
        Self {
            in_fn: in_fn.to_string(),
        }
    }

    pub fn read_lines(&self) -> Result<Vec<SourceLine>> {
        let mut return_lines: Vec<SourceLine> = vec![];

        let lines: Vec<String> = LineParser::parse_lines(&self.in_fn)?;

        for source in lines {
            // get command type
            let cmd_type = self.get_cmd_type(&source);

            // get args
            let args = self.get_args(&source, &cmd_type);

            let mem_seg = self.get_mem_seg(&args, &cmd_type);

            let arith_op = self.get_arith_op(&args, &cmd_type);

            let line = SourceLine::new(&source, args, mem_seg, arith_op, cmd_type);
            return_lines.push(line);
        }
        Ok(return_lines)
    }

    fn get_mem_seg(&self, args: &Args, cmd_type: &CommandType) -> MemSeg {
        let mem_seg = match cmd_type {
            CommandType::POP | CommandType::PUSH => {
                let seg = match args.arg1.as_str() {
                    "local" => MemSeg::LCL,
                    "argument" => MemSeg::ARG,
                    "this" => MemSeg::THIS,
                    "that" => MemSeg::THAT,
                    "constant" => MemSeg::CONST,
                    "static" => MemSeg::STATIC,
                    "temp" => MemSeg::TEMP,
                    "pointer" => MemSeg::PTR,
                    _ => MemSeg::NONE,
                };
                seg
            }
            _ => MemSeg::NONE,
        };
        mem_seg
    }

    fn get_arith_op(&self, args: &Args, cmd_type: &CommandType) -> ArithOp {
        let arith_op = match cmd_type {
            CommandType::ARITHMETIC => {
                let seg = match args.arg1.as_str() {
                    "add" => ArithOp::ADD,
                    "sub" => ArithOp::SUB,
                    "neg" => ArithOp::NEG,
                    "eq" => ArithOp::EQ,
                    "gt" => ArithOp::GT,
                    "lt" => ArithOp::LT,
                    "and" => ArithOp::AND,
                    "or" => ArithOp::OR,
                    "not" => ArithOp::NOT,
                    _ => ArithOp::NONE,
                };
                seg
            }
            _ => ArithOp::NONE,
        };
        arith_op
    }

    fn get_args(&self, source: &str, cmd_type: &CommandType) -> Args {
        let line_spl: Vec<&str> = source.split(' ').collect();
        let args = match cmd_type {
            // arithmetic args
            CommandType::ARITHMETIC => Args {
                arg1: line_spl.get(0).unwrap().to_string(),
                arg2: None,
            },

            // push pop args
            CommandType::POP | CommandType::PUSH | CommandType::FUNCTION | CommandType::CALL => {
                Args {
                    arg1: line_spl.get(1).unwrap().to_string(),
                    arg2: line_spl.get(2).map(|val| val.parse::<i32>().unwrap()),
                    // arg2: match line_spl.get(2) {
                    //     Some(val) => Some(val.parse::<i32>().unwrap()),
                    //     None => None,
                    // },
                }
            }

            // push pop args
            CommandType::IF | CommandType::GOTO | CommandType::LABEL => Args {
                arg1: line_spl.get(1).unwrap().to_string(),
                arg2: None,
            },

            CommandType::RETURN => Args {
                arg1: line_spl.get(0).unwrap().to_string(),
                arg2: None,
            },

            //  => Args {
            //     arg1: line_spl.get(1).unwrap().to_string(),
            //     arg2: None,
            // },

            // unknown args
            _ => Args {
                arg1: "".to_string(),
                arg2: None,
            },
        };
        args
    }

    fn get_cmd_type(&self, source: &str) -> CommandType {
        let line_split: Vec<&str> = source.split(' ').collect();
        let cmd_type = match line_split.len() {
            1 => match line_split.get(0) {
                Some(&"return") => CommandType::RETURN,
                _ => CommandType::ARITHMETIC,
            },
            2 => {
                let c_type = match line_split.get(0) {
                    Some(&"label") => CommandType::LABEL,
                    Some(&"if-goto") => CommandType::IF,
                    Some(&"goto") => CommandType::GOTO,
                    _ => CommandType::UNKOWN,
                };
                c_type
            }
            3 => {
                let c_type = match line_split.get(0) {
                    Some(&"pop") => CommandType::POP,
                    Some(&"push") => CommandType::PUSH,
                    Some(&"call") => CommandType::CALL,
                    Some(&"function") => CommandType::FUNCTION,
                    _ => CommandType::UNKOWN,
                };
                c_type
            }
            // any other command type
            _ => CommandType::UNKOWN,
        };
        cmd_type
    }
}

pub struct LineParser {}

impl LineParser {
    pub fn parse_lines(filename: &str) -> Result<Vec<String>> {
        let file = File::open(filename)?;

        let buf = BufReader::new(file).lines();

        let mut lines: Vec<String> = vec![];

        for source_line in buf.flatten() {
            if source_line.starts_with('/') || source_line.is_empty() {
                continue;
            } else {
                // remove comments
                let no_comment_src = LineParser::strip_comments(&source_line);

                // remove white space
                let words = LineParser::strip_white_space(&no_comment_src);

                // join words
                let final_source = LineParser::join_words(words);
                lines.push(final_source);
            }
        }
        Ok(lines)
    }

    pub fn strip_comments(line: &str) -> String {
        // check if line has comment
        let line_spl: Vec<&str> = line.split("//").collect();
        line_spl.get(0).unwrap().to_string()
        // line.to_string()
    }

    pub fn strip_white_space(line: &str) -> Vec<String> {
        let mut words = vec![];

        for word in line.split(' ') {
            if !word.is_empty() {
                words.push(word.trim().to_string());
            }
        }

        words
    }

    pub fn join_words(words: Vec<String>) -> String {
        words.join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_parser() -> Result<()> {
        // let lines = LineParser::parse_lines("push_const.asm")?;
        Ok(())
    }
}
