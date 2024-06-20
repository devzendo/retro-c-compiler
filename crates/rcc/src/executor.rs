use std::process::{Command, ExitStatus};

use anyhow::{bail, Result};
use mockall::automock;

#[derive(Debug)]
pub struct Execution {
    exit_status: ExitStatus,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl Execution {
    fn code(&self) -> i32 {
        self.exit_status.code().unwrap()
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

#[automock]
pub trait Executor {
    fn run(&mut self, args: Vec<String>) -> Result<Execution>;
}

pub struct CommandExecutor {
}

impl CommandExecutor {
    fn new() -> Self {
        Self {
        }
    }
}

impl Executor for CommandExecutor {
    fn run(&mut self, args: Vec<String>) -> Result<Execution> {
        if args.is_empty() {
            bail!("No command given");
        }
        let args_split = args.split_first().unwrap();
        let output = Command::new(args_split.0).args(args_split.1).output();
        match output {
            Ok(output) => {
                println!("status: {}", output.status);
                let stdout = Some(String::from_utf8_lossy(&output.stdout).to_string());
                println!("stdout: {}", stdout.as_ref().unwrap());
                let stderr = Some(String::from_utf8_lossy(&output.stderr).to_string());
                println!("stderr: {}", stderr.as_ref().unwrap());
                Ok(Execution { exit_status: output.status, stdout: stdout, stderr: stderr })
            }
            Err(err) => bail!("Could not run command '{}': {}", args_split.0, err),
        }
    }

}

#[cfg(test)]
#[path = "./executor_spec.rs"]
mod executor_spec;
