extern crate hamcrest2;

#[cfg(test)]
mod command_line_spec {

    use std::fs::File;

    use hamcrest2::prelude::*;
    use temp_testdir::TempDir;

    use crate::command_line::{parse_command_line, validate_command_line};
    use crate::compiler::TargetPlatform;
    use crate::file_utils_test_helper::temp_config_dir;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn no_file_given() {
        let arg_vec: Vec<&str> = vec!["rcc1"];
        let result = parse_command_line(arg_vec);
        assert_that!(result.is_err(), equal_to(true));
        assert_that!(result.unwrap_err().to_string(), equal_to("error: the following required arguments were not provided:\n  <file>\n\nUsage: rcc1 <file>\n\nFor more information, try '--help'.\n"));
    }

    #[test]
    fn not_a_c_file_given() {
        let arg_vec = vec!["rcc1", "aardvark.stl"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'aardvark.stl' is not a C filename")
        );
    }

    #[test]
    fn nonexistant_lowercase_c_file_given() {
        let arg_vec = vec!["rcc1", "magnumopus.c"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'magnumopus.c' could not be found")
        );
    }

    #[test]
    fn nonexistant_uppercase_c_file_given() {
        let arg_vec = vec!["rcc1", "HELLOWORLD.C"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'HELLOWORLD.C' could not be found")
        );
    }

    #[test]
    fn existing_uppercase_c_file_given() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap()];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(&result, ok());
        assert_that!(
            result.unwrap().c_file.to_str().unwrap(),
            equal_to(c_file.to_str().unwrap())
        );
    }

    #[test]
    fn all_flags_off_by_default() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap()];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.lex, equal_to(false));
        assert_that!(compiler_options.parse, equal_to(false));
        assert_that!(compiler_options.codegen, equal_to(false));
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::Transputer));
    }

    #[test]
    fn lex_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap(), "--lex"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.lex, equal_to(true));
    }

    #[test]
    fn parse_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap(), "--parse"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.parse, equal_to(true));
    }

    #[test]
    fn codegen_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap(), "--codegen"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.codegen, equal_to(true));
    }

    #[test]
    fn architecture_epoc16() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap(), "-a", "EPOC16"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::EPOC16));
    }

    #[test]
    fn architecture_x865_64() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", c_file.to_str().unwrap(), "-a", "X86_64"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::X86_64));
    }

    fn create_file() -> (std::path::PathBuf, TempDir) {
        let (temp, temp_dir) = temp_config_dir();
        let c_file = temp.join("HELLOWORLD.C");
        File::create(c_file.clone()).unwrap();
        (c_file, temp_dir)
    }
}
