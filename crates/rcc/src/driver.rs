use std::path::Path;

#[derive(Debug)]
pub struct DriverOptions {
    c_file: Box<Path>,
    lex: bool,
    parse: bool,
    codegen: bool,
}

struct Driver {}

#[cfg(test)]
#[path = "./driver_spec.rs"]
mod driver_spec;
