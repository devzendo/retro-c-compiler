use log::{debug, info};
/// The DriverController is responsible for running the various stages of the compilation.
/// It orchestrates the various executions using a Driver to run the actual external tools.
/// It is the high level of the driver - for the low level, see Driver.
use sysexits::ExitCode;

use crate::driver::{Driver, DriverOptions};

pub trait DriverController {
    fn drive(
        &self,
        driver_options: DriverOptions,
        driver: Box<dyn Driver>,
    ) -> Result<ExitCode, anyhow::Error>;
}

#[derive(Default)]
pub struct DefaultDriverController {}

impl DefaultDriverController {
}

impl DriverController for DefaultDriverController {
    fn drive(
        &self,
        driver_options: DriverOptions,
        driver: Box<dyn Driver>,
    ) -> Result<ExitCode, anyhow::Error> {
        
        // Preprocess...
        match driver.preprocess() {
            Ok(_success) => {
                debug!("Preprocessor ok");
            }
            Err(err) => {
                anyhow::bail!(format!("Could not run preprocessor: {}", err));
            }
        }

        // Compile...
        match driver.compile() {
            Ok(_success) => {
                debug!("Compiler ok");
            }
            Err(err) => {
                anyhow::bail!(format!("Could not run compiler: {}", err));
            }
        }
        
        if driver_options.stop_after_compilation {
            info!("Stopping after compilation");
            return Ok(ExitCode::Ok);
        } 
            

        // Assemble...
        match driver.assemble() {
            Ok(_success) => {
                debug!("Assembler ok");
            }
            Err(err) => {
                anyhow::bail!(format!("Could not run assembler: {}", err));
            }
        }

        Ok(ExitCode::Ok)
    }
}

#[cfg(test)]
#[path = "./driver_controller_spec.rs"]
mod driver_controller_spec;
