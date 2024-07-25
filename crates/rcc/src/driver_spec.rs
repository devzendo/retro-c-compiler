extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

    use hamcrest2::prelude::*;
    use mockall::*;
    use std::path::PathBuf;

    use crate::driver::{Driver, DefaultDriver, DriverOptions};
    use crate::executor::{Execution, MockExecutor};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn calls_preprocessor() {
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["gcc", "-E", "-P", "file.c", "-o", "file.i"]
            .iter()
            .map(|str| str.to_string())
            .collect();
        let expected_executor_return = Ok(Execution {
            exit_code: Some(0i32),
            stdout: None,
            stderr: None,
        });
        mock_executor
            .expect_run()
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        match sut.preprocess() {
            Ok(success) => {

                assert_eq!(success.exit_code.unwrap(), 0i32);
                assert_that!(success.stdout, none());
                assert_that!(success.stderr, none());
            }
            Err(err) => {
                panic!("was not expecting an error: {}", err);
            }
        }
    }

    #[test]
    fn calls_compiler() {
        // TODO will need revisiting when we have a compiler!
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["rcc1", "file.i", "-o", "file.o"]
            .iter()
            .map(|str| str.to_string())
            .collect();
        let expected_executor_return = Ok(Execution {
            exit_code: Some(0i32),
            stdout: None,
            stderr: None,
        });
        mock_executor
            .expect_run()
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        match sut.compile() {
            Ok(success) => {

                assert_eq!(success.exit_code.unwrap(), 0i32);
                assert_that!(success.stdout, none());
                assert_that!(success.stderr, none());
            }
            Err(err) => {
                panic!("was not expecting an error: {}", err);
            }
        }
    }
}
