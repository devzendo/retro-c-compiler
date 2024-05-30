use std::path::PathBuf;

#[derive(Debug)]
pub struct DriverOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
}

struct Driver {}

#[cfg(test)]
#[path = "./driver_spec.rs"]
mod driver_spec;
