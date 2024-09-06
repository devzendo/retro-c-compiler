extern crate clap;

use std::env;

use log::{debug, info};
use rcc1::command_line::parse_and_validate;
use sysexits::ExitCode;

fn main() -> ExitCode {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "INFO");
    }
    env_logger::init();

    let _compiler_options = match parse_and_validate(env::args_os()) {
        Ok(compiler_options) => {
            debug!("compiler options: {:?}", compiler_options);
            compiler_options
        }
        Err(err) => {
            // --help and --version come in here, and these aren't errors, so log them as info.
            info!("{}", err);
            return ExitCode::Usage;
        }
    };

    info!("Not much to C here yet..");
    ExitCode::Ok
}
