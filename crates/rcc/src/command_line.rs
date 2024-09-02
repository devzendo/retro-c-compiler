use std::{ffi::OsString, path::Path};

use anyhow::{bail, Result};
use clap::{builder::PossibleValue, ValueEnum};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

use crate::driver::{self, DriverOptions, TargetPlatform};

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl ValueEnum for TargetPlatform {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            TargetPlatform::Transputer,
            TargetPlatform::EPOC16,
            TargetPlatform::X86_64,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            TargetPlatform::Transputer => {
                PossibleValue::new("Transputer").help("Parachute Transputer emulator")
            }
            TargetPlatform::EPOC16 => {
                PossibleValue::new("EPOC16").help("Psion EPOC16 (v20) architecture")
            }
            TargetPlatform::X86_64 => PossibleValue::new("X86_64").help("x86_64 architecture"),
        })
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        Self::value_variants()
            .iter()
            .find(|v| {
                v.to_possible_value()
                    .expect("ValueEnum::value_variants contains only values with a corresponding ValueEnum::to_possible_value")
                    .matches(input, ignore_case)
            })
            .cloned()
            .ok_or_else(|| std::format!("Invalid variant: {}", input))
    }
}

impl std::fmt::Display for TargetPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for TargetPlatform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

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
      .arg(
        Arg::new("arch")
          .short('a')
          .long("architecture")
          .help("Choose the target archtecture")
          .value_parser(value_parser!(TargetPlatform)),
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
                    target_platform: *arguments
                        .get_one::<TargetPlatform>("arch")
                        .unwrap_or(&TargetPlatform::Transputer),
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
