use std::path::PathBuf;
use crate::executor::Executor;

#[derive(Debug)]
pub struct DriverOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
}

pub struct Driver {
    driver_options: DriverOptions,
    executor: Box<dyn Executor>,

}

impl Driver {
    pub fn new(driver_options: DriverOptions, executor: Box<dyn Executor>) -> Self {
        Self {
            driver_options,
            executor,
        }
    }
}
#[cfg(test)]
#[path = "./driver_spec.rs"]
mod driver_spec;
