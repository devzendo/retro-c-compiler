extern crate clap;

use std::env;

use log::{debug, error, info};
use rcc::{
    command_line::parse_and_validate,
    driver::DefaultDriver,
    driver_controller::{DefaultDriverController, DriverController},
    executor::CommandExecutor,
};
use sysexits::ExitCode;

fn main() -> ExitCode {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    let driver_options = match parse_and_validate(&mut env::args_os()) {
        Ok(driver_options) => {
            debug!("driver options: {:?}", driver_options);
            driver_options
        }
        Err(err) => {
            // --help and --version come in here, and these aren't errors, so log them as info.
            info!("{}", err);
            return ExitCode::Usage;
        }
    };

    let command_executor = CommandExecutor::default();
    let driver = DefaultDriver::new(driver_options.clone(), Box::new(command_executor));
    let driver_controller = DefaultDriverController::new();
    match driver_controller.drive(driver_options, Box::new(driver)) {
        Ok(code) => {
            debug!("Exiting with code {code}");
            return code;
        }
        Err(err) => {
            error!("Exiting with error {err}");
            return ExitCode::Unavailable;
        }
    }
}
