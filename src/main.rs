#![allow(clippy::upper_case_acronyms)]

use std::io::Result;
use std::{env, fs, fs::metadata, path::Path};

mod asm;
mod code;
mod line;
mod parser;

use asm::Asm;
use code::{Code, CodeWriter};
use parser::Parser;

pub fn gen_init_asm(dir_name: &str) -> Vec<Asm> {
    let mut asm_lines: Vec<Asm> = vec![];

    // check if Sys.vm exists within directory
    if Path::new(&format!("./{}/{}", dir_name, "Sys.vm")).exists() {
        let mut code = Code::new("SysInitBootstrap");
        let bootstrap = code.gen_init_asm();

        asm_lines.push(bootstrap);

        // write Sys init to asm first
        let mut sys_init_asm = gen_asm(&format!("./{}/{}", dir_name, "Sys.vm")).unwrap();

        asm_lines.append(&mut sys_init_asm);
    }

    asm_lines
}

pub fn gen_asm(filename: &str) -> Result<Vec<Asm>> {
    let parser = Parser::new(filename);

    let src_lines = parser.read_lines()?;

    let mut asm_ins: Vec<Asm> = Vec::new();

    let base_fn = Path::new(&filename).file_name().unwrap().to_str().unwrap();

    let base_fn = base_fn.split('.').collect::<Vec<&str>>();

    let mut code = Code::new(base_fn.get(0).unwrap());

    for line in src_lines {
        let asm = code.gen_asm(&line);
        asm_ins.push(asm);
    }

    Ok(asm_ins)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Main logic to run program
    if let Some(filename) = args.get(1) {
        // check if filename is directory or string
        let meta = metadata(filename).unwrap();

        let asm_ins = if meta.is_dir() {
            let paths = fs::read_dir(format!("./{}", filename)).unwrap();

            let mut asm_lines = gen_init_asm(filename);

            // walk directory
            for entry in paths {
                let dir_entry = entry?;
                let path = dir_entry.path();
                let filename = path.to_str().unwrap();

                // skip Sys.vm
                if filename.contains("Sys.vm") {
                    continue;
                }

                // only handle vm extensions
                if filename.ends_with("vm") {
                    let mut file_asm = gen_asm(filename)?;

                    // append file asm to global asm
                    asm_lines.append(&mut file_asm);
                }
            }
            asm_lines
        } else {
            gen_asm(filename)?
        };

        let mut writer = CodeWriter::new(filename)?;

        for asm in asm_ins {
            writer.write_asm(&asm)?;
        }
    } else {
        println!("There was an error parsing the args, please provide filename")
    }

    Ok(())
}
