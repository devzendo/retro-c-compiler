extern crate clap;

use std::{env, ffi::OsString, path::Path};

use anyhow::{bail, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use driver::{DefaultDriver, DriverOptions};
use driver_controller::{DefaultDriverController, DriverController};
use executor::CommandExecutor;
use log::{debug, error, info};
use sysexits::ExitCode;

mod driver;
mod driver_controller;
mod executor;
mod file_utils;
mod suffix_translator;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_command_line<I, T>(itr: I) -> Result<ArgMatches, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    Command::new("rcc")
      .version(VERSION)
      .author("DevZendo.org")
      .about("Transputer & EPOC16 C Compiler")
      .arg(
        Arg::new("file")
          .help("The path (absolute or relative) of a C file to compile")
          .required(true) // nice, but causes termination with a less-than-perfect error, and we want to test for its absence
      )
      .arg(
        Arg::new("lex")
          .short('l')
          .long("lex")
          .help("Run the lexer but stop before parsing")
          .action(ArgAction::SetTrue)
      )
      .arg(
        Arg::new("parse")
          .short('p')
          .long("parse")
          .help("Run the lexer and parser, but stop before assembly generation")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("codegen")
          .short('c')
          .long("codegen")
          .help("Run the lexer, parser, and assembly generation, but stop before code generation")
          .action(ArgAction::SetTrue),
      )
      .try_get_matches_from(itr)
}

fn validate_command_line(arguments: ArgMatches) -> Result<driver::DriverOptions> {
    let file = arguments.get_one::<String>("file");
    match file {
        Some(file) => {
            if file.to_lowercase().ends_with(".c") {
                let file_path = Path::new(file);
                if !file_path.exists() {
                    bail!(format!("'{}' could not be found", file));
                }
                Ok(DriverOptions {
                    c_file: Box::new(file_path.to_owned()),
                    lex: arguments.get_flag("lex"),
                    parse: arguments.get_flag("parse"),
                    codegen: arguments.get_flag("codegen"),
                })
            } else {
                bail!(format!("'{}' is not a C filename", file))
            }
        }
        None => bail!("C filename not supplied"),
    }
}

fn parse_and_validate<I, T>(itr: I) -> Result<driver::DriverOptions>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let parsed = parse_command_line(itr)?;
    validate_command_line(parsed)
}

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
            return ExitCode::Usage
        }
    };

    let command_executor = CommandExecutor::default();
    let driver = DefaultDriver::new(driver_options.clone(), Box::new(command_executor));
    let driver_controller = DefaultDriverController::new();
    match driver_controller.drive(driver_options, Box::new(driver)) {
        Ok(code) => {
            debug!("Exiting with code {code}");
            return code;
        },
        Err(err) => {
            error!("Exiting with error {err}");
            return ExitCode::Unavailable;
        },
    }
}

#[cfg(test)]
#[path = "./main_spec.rs"]
mod main_spec;
