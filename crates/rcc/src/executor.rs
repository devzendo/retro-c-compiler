/// The Executor allows the running of external programs and handling their exit codes and outputs.

use std::process::Command;

use anyhow::{bail, Result};

use log::{debug, error};
#[cfg(test)]
use mockall::automock;

#[derive(Debug, Clone)]
pub struct Execution {
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Execution {
    pub fn code(&self) -> Option<i32> {
        self.exit_code
    }

    pub fn stdout(&self) -> String {
        self.stdout
            .as_ref()
            .map_or_else(|| "".to_owned(), |s| s.clone())
    }

    pub fn stderr(&self) -> String {
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
                let stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
                let stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
                if output.status.code().unwrap_or(1) == 0 {
                    debug!("status: {}", output.status);
                    debug!("stdout: {}", stdout.as_ref().unwrap());
                    debug!("stderr: {}", stderr.as_ref().unwrap());
                } else {
                    error!("Execution failure of {:?}", args.clone().join(" "));
                    error!("status: {}", output.status);
                    error!("stdout: {}", stdout.as_ref().unwrap());
                    error!("stderr: {}", stderr.as_ref().unwrap());
                }
                Ok(Execution { exit_code: output.status.code(), stdout: stdout, stderr: stderr })
            }
            Err(err) => bail!("Could not run command '{}': {}", args_split.0, err),
        }
    }

}

#[cfg(test)]
#[path = "./executor_spec.rs"]
mod executor_spec;
