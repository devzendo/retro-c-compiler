extern crate hamcrest2;

#[cfg(test)]
mod driver_controller_spec {

    use anyhow::bail;
    use sysexits::ExitCode;
    use std::path::PathBuf;

    use crate::driver::{DriverOptions, MockDriver, TargetPlatform};
    use crate::driver_controller::{DefaultDriverController, DriverController};
    use crate::executor::Execution;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn driver_options() -> DriverOptions {
        let c_file = PathBuf::from("file.c");
        DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            target_platform: TargetPlatform::Transputer,
        }
    }
    
    #[test]
    fn calls_phases_happy_path() {
        let mut mock_driver = MockDriver::new();
        let expected_preprocessor_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_preprocess().times(1).return_once(move || expected_preprocessor_return);
        let expected_compiler_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_compile().times(1).return_once(move || expected_compiler_return);
        let expected_assembler_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_assemble().times(1).return_once(move || expected_assembler_return);
        let driver_options = driver_options();

        let sut = DefaultDriverController::new();
        let res = sut.drive(driver_options, Box::new(mock_driver));

        let exit_code = res.ok().unwrap();
        assert_eq!(exit_code, ExitCode::Ok);
    }

    #[test]
    fn preprocessor_fails() {
        let mut mock_driver = MockDriver::new();
        mock_driver.expect_preprocess().return_once(move || bail!("Preprocessor failed"));
        let driver_options = driver_options();

        let sut = DefaultDriverController::new();
        let res = sut.drive(driver_options, Box::new(mock_driver));

        let msg = res.err().unwrap().to_string();
        assert_eq!(msg, "Could not run preprocessor: Preprocessor failed");
    }

    #[test]
    fn compiler_fails() {
        let mut mock_driver = MockDriver::new();
        let expected_preprocessor_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_preprocess().return_once(move || expected_preprocessor_return);
        mock_driver.expect_compile().return_once(move || bail!("Compiler failed"));
        let driver_options = driver_options();

        let sut = DefaultDriverController::new();
        let res = sut.drive(driver_options, Box::new(mock_driver));

        let msg = res.err().unwrap().to_string();
        assert_eq!(msg, "Could not run compiler: Compiler failed");
    }

    #[test]
    fn assembler_fails() {
        let mut mock_driver = MockDriver::new();
        let expected_preprocessor_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_preprocess().return_once(move || expected_preprocessor_return);
        let expected_compiler_return: Result<Execution, anyhow::Error> = Ok(Execution { exit_code: Some(0), stdout: None, stderr: None });
        mock_driver.expect_compile().return_once(move || expected_compiler_return);
        mock_driver.expect_assemble().return_once(move || bail!("Assembler failed"));
        let driver_options = driver_options();

        let sut = DefaultDriverController::new();
        let res = sut.drive(driver_options, Box::new(mock_driver));

        let msg = res.err().unwrap().to_string();
        assert_eq!(msg, "Could not run assembler: Assembler failed");
    }
}
