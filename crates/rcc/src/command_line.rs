use std::{ffi::OsString, path::Path};

use anyhow::{bail, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::driver::{self, DriverOptions};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_command_line<I, T>(itr: I) -> Result<ArgMatches, clap::Error>
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
      .arg(
        Arg::new("save-temps")
          .short('s')
          .long("save-temps")
          .help("Do not delete temporary preprocessor and assembly files")
          .action(ArgAction::SetTrue)
      )
       .try_get_matches_from(itr)
}

pub fn validate_command_line(arguments: ArgMatches) -> Result<driver::DriverOptions> {
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
                    save_temps: arguments.get_flag("save-temps"),
                })
            } else {
                bail!(format!("'{}' is not a C filename", file))
            }
        }
        None => bail!("C filename not supplied"),
    }
}

pub fn parse_and_validate<I, T>(itr: I) -> Result<driver::DriverOptions>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let parsed = parse_command_line(itr)?;
    validate_command_line(parsed)
}

#[cfg(test)]
#[path = "./command_line_spec.rs"]
pub mod command_line_spec;