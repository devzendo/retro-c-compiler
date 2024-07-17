/// The Executor allows the running of external programs and handling their exit codes and outputs.

use std::process::Command;

use anyhow::{bail, Result};

use log::debug;
#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone)]
pub struct Execution {
    pub(crate) exit_code: Option<i32>,
    pub(crate) stdout: Option<String>,
    pub(crate) stderr: Option<String>,
}

impl Execution {
    fn code(&self) -> Option<i32> {
        self.exit_code
    }

    fn stdout(&self) -> String {
        self.stdout
            .as_ref()
            .map_or_else(|| "".to_owned(), |s| s.clone())
    }

    fn stderr(&self) -> String {
        self.stderr
            .as_ref()
            .map_or_else(|| "".to_owned(), |s| s.clone())
    }
}

#[cfg_attr(test, automock)]
pub trait Executor {
    fn run(&self, args: Vec<String>) -> Result<Execution>;
}

#[derive(Default)]
pub struct CommandExecutor {
}

impl Executor for CommandExecutor {
    fn run(&self, args: Vec<String>) -> Result<Execution> {
        if args.is_empty() {
            bail!("No command given");
        }
        let args_split = args.split_first().unwrap();
        debug!("Executing {:?}", args.clone().join(" "));
        let output = Command::new(args_split.0).args(args_split.1).output();
        // I could just capture the Output, rather than encapsulating it in the fields of the Execution,
        // however, Execution is our abstraction, can be constructed; Output, perhaps not so easy? And
        // mocking the Executor, and trying to create an arbitrary ExitStatus proved impossible. So,
        // the Execution abstraction is specifically easy to test with, encapuslating hard-to-create internals.
        match output {
            Ok(output) => {
                debug!("status: {}", output.status);
                let stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
                debug!("stdout: {}", stdout.as_ref().unwrap());
                let stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
                debug!("stderr: {}", stderr.as_ref().unwrap());
                Ok(Execution { exit_code: output.status.code(), stdout: stdout, stderr: stderr })
            }
            Err(err) => bail!("Could not run command '{}': {}", args_split.0, err),
        }
    }

}

#[cfg(test)]
#[path = "./executor_spec.rs"]
mod executor_spec;
