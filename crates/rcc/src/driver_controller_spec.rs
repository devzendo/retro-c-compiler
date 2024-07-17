extern crate hamcrest2;

#[cfg(test)]
mod driver_controller_spec {

    use hamcrest2::prelude::*;
    use sysexits::ExitCode;
    use std::path::PathBuf;

    use crate::driver::{DriverOptions, MockDriver};
    use crate::driver_controller::{DefaultDriverController, DriverController};
    use crate::executor::Execution;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn calls_preprocessor() {
        let mut mock_driver = MockDriver::new();
        let expected_driver_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_preprocess().return_once(move || expected_driver_return);

        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
        };
        let sut = DefaultDriverController::new();
        let res = sut.drive(driver_options, Box::new(mock_driver));
        assert_that!(&res, ok());
        let exec = res.ok().unwrap();
        assert_eq!(exec, ExitCode::Ok);
    }
}
