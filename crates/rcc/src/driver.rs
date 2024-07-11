use std::path::PathBuf;
use crate::{executor::{Execution, Executor}, suffix_translator::SuffixTranslator};

#[derive(Debug)]
pub struct DriverOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
}

pub trait Driver {
    fn preprocess(&self) -> Result<Execution, anyhow::Error>;
}

pub struct DefaultDriver {
    driver_options: DriverOptions,
    executor: Box<dyn Executor>,

}

impl DefaultDriver {
    pub fn new(driver_options: DriverOptions, executor: Box<dyn Executor>) -> Self {
        Self {
            driver_options,
            executor,
        }
    }
}

impl Driver for DefaultDriver {
    fn preprocess(&self) -> Result<Execution, anyhow::Error> {
        let xlat = SuffixTranslator::new(self.driver_options.c_file.to_path_buf());
        // TODO: CROSSPLATFORM EPOC16
        // TODO move this conversion mess into driver options...
        let preprocessor = &xlat.preprocessor();
        let preprocessor_file = preprocessor.as_os_str().to_string_lossy();
        let c_file = self.driver_options.c_file.as_os_str().to_string_lossy();
        let args: Vec<String> = vec!["gcc", "-E", "-P", &c_file, "-o", &preprocessor_file].iter().map(|str| str.to_string()).collect();

        self.executor.run(args)
    }
}

#[cfg(test)]
#[path = "./driver_spec.rs"]
mod driver_spec;
