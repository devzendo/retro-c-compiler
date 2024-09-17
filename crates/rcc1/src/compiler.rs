use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use chumsky::prelude::*;
use log::{debug, error, info};
use common::target_platform::TargetPlatform;
use sysexits::ExitCode;
use crate::lexer::lexer;

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub c_file: Box<PathBuf>,
    pub asm_file: Option<Box<PathBuf>>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
    pub target_platform: TargetPlatform,
}

pub struct Compiler {

}

impl Compiler {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn compile(&self, options: CompilerOptions) -> Result<ExitCode, anyhow::Error> {
        debug!("Loading {}", options.c_file.display());
        let mut file = File::open(options.c_file.as_path()).unwrap();
        // TODO what if the file can't be opened, eg permissions. Precondition is that it exists
        // as checked by the command line.
        let file_size = file.metadata().unwrap().len();
        let mut input_buffer = String::with_capacity(file_size as usize);
        let read_bytes = file.read_to_string(&mut input_buffer).unwrap();
        debug!("Read {} bytes", read_bytes);
        // TODO handle read failures

        let lexer_start = std::time::Instant::now();
        let lexer = lexer();
        let lexer_duration = lexer_start.elapsed();
        debug!("Lexer creation took {:?}μs", lexer_duration.as_micros());

        // Test harness lexical analysis check...
        if options.lex {
            let lexical_analysis_start = std::time::Instant::now();
            let parse_result = lexer.check(&*input_buffer);
            // debug!("Parse result: {:?}", parse_result);
            let errs = parse_result.into_output_errors().1;
            let lexical_analysis_duration = lexical_analysis_start.elapsed();
            debug!("Lexical analysis took {:?}μs", lexical_analysis_duration.as_micros());
            if errs.is_empty() {
                info!("Lexical analysis successful");
                return Ok(ExitCode::Ok);
            } else {
                error!("Lexical analysis unsuccessful");
                errs.into_iter().for_each(|e| error!("{:?}", e));
                return Ok(ExitCode::DataErr);
            }
        }
        //
        // let (tokens, errs) = lexer.parse(&*input_buffer).into_output_errors();
        // debug!("Lexed tokens: {:#?}", tokens);
        // errs.into_iter().for_each(|e| error!("{:?}", e));

        Ok(ExitCode::Ok)
    }
}

#[cfg(test)]
#[path = "./compiler_spec.rs"]
pub mod compiler_spec;
