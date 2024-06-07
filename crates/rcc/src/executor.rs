use std::process::{Command, ExitStatus};

use anyhow::{bail, Result};
use mockall::automock;

#[automock]
pub trait Executor {
    fn run(&mut self) -> Result<ExitStatus>;
    fn stdout(&self) -> String;
    fn stderr(&self) -> String;
}

pub struct CommandExecutor {
    args: Vec<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl CommandExecutor {
    fn new(args: Vec<String>) -> Self {
        Self {
            args,
            stdout: None,
            stderr: None,
        }
    }
}

impl Executor for CommandExecutor {
    fn run(&mut self) -> Result<ExitStatus> {
        if self.args.is_empty() {
            bail!("No command given");
        }
        let args_split = self.args.split_first().unwrap();
        let output = Command::new(args_split.0).args(args_split.1).output();
        match output {
            Ok(output) => {
                println!("status: {}", output.status);
                self.stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
                println!("stdout: {}", self.stdout.as_ref().unwrap());
                self.stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
                println!("stderr: {}", self.stderr.as_ref().unwrap());
                Ok(output.status)
            }
            Err(err) => bail!("Could not run command '{}': {}", args_split.0, err),
        }
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

#[cfg(test)]
#[path = "./executor_spec.rs"]
mod executor_spec;
