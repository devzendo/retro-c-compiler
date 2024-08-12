extern crate hamcrest2;
extern crate log;
extern crate testing_logger;

#[cfg(test)]
mod executor_itest {

    use hamcrest2::prelude::*;
    use log::Level;
    use rcc::executor::{CommandExecutor, Executor};

    // TODO need a test that captures logging, and if there's an error from the command, display the error/output as ERROR level
    // rather than just DEBUG.

    #[test]
    #[serial_test::serial]
    fn command_succeeds_and_logs_stdout_and_stderr_at_debug() {
        testing_logger::setup();
        // TODO: CROSSPLATFORM
        let e = CommandExecutor::default();
        let ex = e.run(vec!["true".to_owned()]);
        let es = ex.ok().unwrap();
        assert_that!(es.code().unwrap(), equal_to(0));
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 4, "did not receive the expected number of log lines");
            assert_eq!(captured_logs[0].body, "Executing \"true\"");
            assert_eq!(captured_logs[0].level, Level::Debug);
            assert_eq!(captured_logs[1].body, "status: exit status: 0");
            assert_eq!(captured_logs[1].level, Level::Debug);
            assert_eq!(captured_logs[2].body, "stdout: ");
            assert_eq!(captured_logs[2].level, Level::Debug);
            assert_eq!(captured_logs[3].body, "stderr: ");
            assert_eq!(captured_logs[3].level, Level::Debug);
        });
    }

    #[test]
    #[serial_test::serial]
    fn command_fails_and_logs_stdout_and_stderr() {
        testing_logger::setup();
        // TODO: CROSSPLATFORM
        let e = CommandExecutor::default();
        let ex = e.run(vec!["cat".to_owned(), "nonexistant.txt".to_owned()]);
        let es = ex.ok().unwrap();
        assert_that!(es.code().unwrap(), equal_to(1));
        testing_logger::validate( |captured_logs| {
            assert_eq!(captured_logs.len(), 5);
            assert_eq!(captured_logs[0].body, "Executing \"cat nonexistant.txt\"");
            assert_eq!(captured_logs[0].level, Level::Debug);
            assert_eq!(captured_logs[1].body, "Execution failure of \"cat nonexistant.txt\"");
            assert_eq!(captured_logs[1].level, Level::Error);
            assert_eq!(captured_logs[2].body, "status: exit status: 1");
            assert_eq!(captured_logs[2].level, Level::Error);
            assert_eq!(captured_logs[3].body, "stdout: ");
            assert_eq!(captured_logs[3].level, Level::Error);
            assert_eq!(captured_logs[4].body, "stderr: cat: nonexistant.txt: No such file or directory\n");
            assert_eq!(captured_logs[4].level, Level::Error);
        });
    }

}
