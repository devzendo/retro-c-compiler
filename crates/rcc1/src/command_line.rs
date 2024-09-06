use std::{ffi::OsString, path::Path};

use anyhow::{bail, Result};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use common::target_platform::TargetPlatform;

use crate::compiler::CompilerOptions;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn parse_command_line<I, T>(itr: I) -> Result<ArgMatches, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    Command::new("rcc1")
        .version(VERSION)
        .author("DevZendo.org")
        .about("Transputer & EPOC16 C Compiler (Back End)")
        .arg(
            Arg::new("file")
                .help("The path (absolute or relative) of a preprocessed C file to compile (.i)")
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
            Arg::new("arch")
                .short('a')
                .long("architecture")
                .help("Choose the target archtiecture")
                .value_parser(value_parser!(TargetPlatform)),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("The path (absolute or relative) of the output assembler file (.asm)")
                // Not making this required, as the test harness will want to run just the lex/parse/codegen without output.
        )
        .try_get_matches_from(itr)
}

pub fn validate_command_line(arguments: ArgMatches) -> Result<CompilerOptions> {
    match arguments.get_one::<String>("file") {
        Some(file) => {
            if file.to_lowercase().ends_with(".i") {
                let file_path = Path::new(file);
                if !file_path.exists() {
                    bail!(format!("'{}' could not be found", file));
                }
                // There may be an output file.
                let asm_file = match arguments.get_one::<String>("output") {
                    Some(o) => {
                        if o.to_lowercase().ends_with(".asm") {
                            Some(Box::new(Path::new(o).to_owned()))
                        } else {
                            bail!("'{}' is not an assembler file (.asm)", o);
                        }
                    },
                    None => None,
                };
                Ok(CompilerOptions {
                    c_file: Box::new(file_path.to_owned()),
                    asm_file,
                    lex: arguments.get_flag("lex"),
                    parse: arguments.get_flag("parse"),
                    codegen: arguments.get_flag("codegen"),
                    target_platform: *arguments
                        .get_one::<TargetPlatform>("arch")
                        .unwrap_or(&TargetPlatform::Transputer),
                })
            } else {
                bail!("'{}' is not a preprocessed C filename (.i)", file)
            }
        }
        None => bail!("preprocessed C filename (.i) not supplied"),
    }
}

pub fn parse_and_validate<I, T>(itr: I) -> Result<CompilerOptions>
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
