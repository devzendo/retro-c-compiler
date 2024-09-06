/// The Driver is responsible for running specific external processes with the relevant arguments.
/// It's the lower level of the driver - for the higher level, see the DriverController.

use std::path::PathBuf;
use crate::{executor::{Execution, Executor}, suffix_translator::SuffixTranslator};

use common::target_platform::TargetPlatform;
use log::{debug, warn};
#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone)]
pub struct DriverOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
    pub save_temps: bool,
    pub stop_after_compilation: bool,
    pub target_platform: TargetPlatform,
}

#[cfg_attr(test, automock)]
pub trait Driver {
    fn preprocess(&self) -> Result<Execution, anyhow::Error>;
    fn compile(&self) -> Result<Execution, anyhow::Error>;
    fn assemble(&self) -> Result<Execution, anyhow::Error>;
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
    
    fn compile(&self) -> Result<Execution,anyhow::Error> {
        // TODO don't know what the actual command line will be just yet, so this is made up..
        let xlat = SuffixTranslator::new(self.driver_options.c_file.to_path_buf());
        // TODO: CROSSPLATFORM EPOC16
        // TODO move this conversion mess into driver options...
        let preprocessor = &xlat.preprocessor();
        let preprocessor_file = preprocessor.as_os_str().to_string_lossy();
        if !preprocessor.exists() {
            warn!("Preprocessed file {} does not exist", preprocessor_file);
            // is there any point running the compiler in this case?
        }
        let assembly = &xlat.assembler();
        let assembly_file = assembly.as_os_str().to_string_lossy();
        let mut args: Vec<String> = vec!["rcc1".to_string()];
        if self.driver_options.lex {
            args.push("--lex".to_string())
        }
        if self.driver_options.parse {
            args.push("--parse".to_string())
        }
        if self.driver_options.codegen {
            args.push("--codegen".to_string())
        }
        if self.driver_options.target_platform != TargetPlatform::Transputer {
            args.push("--architecture".to_string());
            let name = self.driver_options.target_platform.to_string();
            args.push(name);
        }
        let mut rest: Vec<String> = vec![preprocessor_file.to_string(), "-o".to_string(), assembly_file.to_string()];
        args.append(&mut rest);

        let result = self.executor.run(args.iter().map(|str| str.to_string()).collect());
        // tidy up after the preprocessor unless requested
        if self.driver_options.save_temps {
            debug!("Retaining temporary preprocessor file {}", preprocessor_file);
        } else {
            match std::fs::remove_file(preprocessor) {
                Ok(_) => debug!("Removed preprocessor file {}", preprocessor_file),
                Err(e) => warn!("Could not remove preprocessor file {}: {}", preprocessor_file, e),
            }
        }
        result
    }
    
    fn assemble(&self) -> Result<Execution,anyhow::Error> {
        let xlat = SuffixTranslator::new(self.driver_options.c_file.to_path_buf());
        // TODO: CROSSPLATFORM EPOC16
        let assembly = &xlat.assembler();
        let assembly_file = assembly.as_os_str().to_string_lossy();
        let binary = &xlat.binary();
        let binary_file = binary.as_os_str().to_string_lossy();
        let listing = &xlat.listing();
        let listing_file = listing.as_os_str().to_string_lossy();
        let args: Vec<String> = vec!["tmasm", &assembly_file, "-o", &binary_file, "-l", &listing_file].iter().map(|str| str.to_string()).collect();
    
        let result = self.executor.run(args);
        // tidy up after the assembler unless requested
        if self.driver_options.save_temps {
            debug!("Retaining temporary assembler file {}", assembly_file);
        } else {
            match std::fs::remove_file(assembly) {
                Ok(_) => debug!("Removed assembler file {}", assembly_file),
                Err(e) => warn!("Could not remove assembler file {}: {}", assembly_file, e),
            }
        }
        result
    }
}

#[cfg(test)]
#[path = "./driver_spec.rs"]
mod driver_spec;
