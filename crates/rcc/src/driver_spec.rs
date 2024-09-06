extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

    use common_test::file_utils_test_helper::temp_config_dir;
    use hamcrest2::prelude::*;
    use mockall::*;
    use std::fs::File;
    use std::path::PathBuf;

    use crate::driver::{DefaultDriver, Driver, DriverOptions, TargetPlatform};
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.preprocess());
    }

    fn execution_ok(result: Result<Execution, anyhow::Error>) {
        match result {
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
        let expected_executor_args: Vec<String> = vec!["rcc1", "file.i", "-o", "file.asm"]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.compile());
    }

    #[test]
    fn flags_passed_to_compiler() {
        // TODO will need revisiting when we have a compiler!
        let driver_options = DriverOptions {
            c_file: Box::new(PathBuf::from("file.c")),
            lex: true,
            parse: true,
            codegen: true,
            save_temps: false,              // These two aren't passed through
            stop_after_compilation: false,  // These two aren't passed through
            target_platform: TargetPlatform::Transputer,
        };
        let expected_args = vec!["rcc1", "--lex", "--parse", "--codegen", "file.i", "-o", "file.asm"];
        check_compiler_flags(driver_options, &expected_args);
    }

    fn check_compiler_flags(driver_options: DriverOptions, expected_args: &Vec<&str>) {
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = expected_args
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.compile());
    }

    #[test]
    fn x86_64_architecture_passed_to_compiler() {
        // TODO will need revisiting when we have a compiler!
        let expected_args = vec!["rcc1", "--architecture", "X86_64", "file.i", "-o", "file.asm"];
        let driver_options = DriverOptions {
            c_file: Box::new(PathBuf::from("file.c")),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::X86_64,
        };
        check_compiler_flags(driver_options, &expected_args);
    }

    #[test]
    fn epoc16_architecture_passed_to_compiler() {
        // TODO will need revisiting when we have a compiler!
        let expected_args = vec!["rcc1", "--architecture", "EPOC16", "file.i", "-o", "file.asm"];
        let driver_options = DriverOptions {
            c_file: Box::new(PathBuf::from("file.c")),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::EPOC16,
        };
        check_compiler_flags(driver_options, &expected_args);
    }

    #[test]
    fn transputer_architecture_is_default_and_not_passed_to_compiler() {
        // TODO will need revisiting when we have a compiler!
        let expected_args = vec!["rcc1", "file.i", "-o", "file.asm"];
        let driver_options = DriverOptions {
            c_file: Box::new(PathBuf::from("file.c")),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };
        check_compiler_flags(driver_options, &expected_args);
    }

    #[test]
    fn calls_assembler() {
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["tmasm", "file.asm", "-o", "file.bin", "-l", "file.lst"]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.assemble());
    }

    #[test]
    fn preprocessor_file_deleted_after_compilation() {
        let (temp, _temp_dir) = temp_config_dir();
        let i_file = temp.join("file.i");
        File::create(i_file.clone()).unwrap();
        let i_file_absolute = i_file.as_os_str().to_str().unwrap();
        assert!(i_file.exists(), "temp preprocessor file was not created");
        let asm_file = temp.join("file.asm");
        let asm_file_absolute = asm_file.as_os_str().to_str().unwrap();

        // Pretend to run the compiler.
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["rcc1", i_file_absolute, "-o", asm_file_absolute]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = temp.join("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.compile());
        assert!(!i_file.exists(), "temp preprocessor file was not deleted by driver");
    }

    #[test]
    fn preprocessor_file_retained_after_compilation_with_save_temps() {
        let (temp, _temp_dir) = temp_config_dir();
        let i_file = temp.join("file.i");
        File::create(i_file.clone()).unwrap();
        let i_file_absolute = i_file.as_os_str().to_str().unwrap();
        assert!(i_file.exists(), "temp preprocessor file was not created");
        let asm_file = temp.join("file.asm");
        let asm_file_absolute = asm_file.as_os_str().to_str().unwrap();

        // Pretend to run the compiler.
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["rcc1", i_file_absolute, "-o", asm_file_absolute]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = temp.join("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: true,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.compile());
        assert!(i_file.exists(), "temp preprocessor file was deleted by driver");
    }


    // TODO preprocessor file not deleted if compiler fails?

    #[test]
    fn assembly_file_deleted_after_assembly() {
        let (temp, _temp_dir) = temp_config_dir();
        let asm_file = temp.join("file.asm");
        File::create(asm_file.clone()).unwrap();
        let asm_file_absolute = asm_file.as_os_str().to_str().unwrap();
        assert!(asm_file.exists(), "temp assembly file was not created");
        let bin_file = temp.join("file.bin");
        let bin_file_absolute = bin_file.as_os_str().to_str().unwrap();
        let lst_file = temp.join("file.lst");
        let lst_file_absolute = lst_file.as_os_str().to_str().unwrap();

        // Pretend to run the assembler.
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["tmasm", asm_file_absolute, "-o", bin_file_absolute, "-l", lst_file_absolute]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = temp.join("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: false,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.assemble());
        assert!(!asm_file.exists(), "temp assembly file was not deleted by driver");
    }

    #[test]
    fn assembly_file_retained_after_assembly_with_save_temps() {
        let (temp, _temp_dir) = temp_config_dir();
        let asm_file = temp.join("file.asm");
        File::create(asm_file.clone()).unwrap();
        let asm_file_absolute = asm_file.as_os_str().to_str().unwrap();
        assert!(asm_file.exists(), "temp assembly file was not created");
        let bin_file = temp.join("file.bin");
        let bin_file_absolute = bin_file.as_os_str().to_str().unwrap();
        let lst_file = temp.join("file.lst");
        let lst_file_absolute = lst_file.as_os_str().to_str().unwrap();

        // Pretend to run the assembler.
        let mut mock_executor = MockExecutor::new();
        let expected_executor_args: Vec<String> = vec!["tmasm", asm_file_absolute, "-o", bin_file_absolute, "-l", lst_file_absolute]
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
            .times(1)
            .with(predicate::eq(expected_executor_args))
            .return_once(move |_| expected_executor_return);
        let c_file = temp.join("file.c");
        let driver_options = DriverOptions {
            c_file: Box::new(c_file),
            lex: false,
            parse: false,
            codegen: false,
            save_temps: true,
            stop_after_compilation: false,
            target_platform: TargetPlatform::Transputer,
        };

        let sut = DefaultDriver::new(driver_options, Box::new(mock_executor));
        execution_ok(sut.assemble());
        assert!(asm_file.exists(), "temp assembly file was deleted by driver but save-temps given");
    }
}
