extern crate hamcrest2;

#[cfg(test)]
mod main_spec {

    use std::{fs::File, path::Path};

    use hamcrest2::prelude::*;
    use temp_testdir::TempDir;

    use crate::{parse_command_line, validate_command_line};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn temp_config_dir() -> (Box<Path>, TempDir) {
        // Return both objects as if temp_dir is not moved back to the caller, it'll drop and
        // delete.
        let temp_dir = TempDir::default();
        let temp = temp_dir.to_path_buf();
        assert_that!(temp.as_path(), dir_exists());

        (temp.into_boxed_path(), temp_dir)
    }

    #[test]
    fn no_file_given() {
        let arg_vec: Vec<&str> = vec!["rcc"];
        let result = parse_command_line(arg_vec);
        assert_that!(result.is_err(), equal_to(true));
        assert_that!(result.unwrap_err().to_string(), equal_to("error: The following required arguments were not provided:\n    <file>\n\nUSAGE:\n    rcc [OPTIONS] <file>\n\nFor more information try --help\n"));
    }

    #[test]
    fn not_a_c_file_given() {
        let arg_vec = vec!["rcc", "aardvark.stl"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'aardvark.stl' is not a C filename")
        );
    }

    #[test]
    fn nonexistant_lowercase_c_file_given() {
        let arg_vec = vec!["rcc", "magnumopus.c"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'magnumopus.c' could not be found")
        );
    }

    #[test]
    fn nonexistant_uppercase_c_file_given() {
        let arg_vec = vec!["rcc", "HELLOWORLD.C"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'HELLOWORLD.C' could not be found")
        );
    }

    #[test]
    fn existing_uppercase_c_file_given() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc", c_file.to_str().unwrap()];
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

        let arg_vec = vec!["rcc", c_file.to_str().unwrap()];
        let driver_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(driver_options.lex, equal_to(false));
        assert_that!(driver_options.parse, equal_to(false));
        assert_that!(driver_options.codegen, equal_to(false));
    }

    #[test]
    fn lex_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc", c_file.to_str().unwrap(), "--lex"];
        let driver_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(driver_options.lex, equal_to(true));
        assert_that!(driver_options.parse, equal_to(false));
        assert_that!(driver_options.codegen, equal_to(false));
    }

    #[test]
    fn parse_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc", c_file.to_str().unwrap(), "--parse"];
        let driver_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(driver_options.lex, equal_to(false));
        assert_that!(driver_options.parse, equal_to(true));
        assert_that!(driver_options.codegen, equal_to(false));
    }

    #[test]
    fn codegen_flag() {
        let (c_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc", c_file.to_str().unwrap(), "--codegen"];
        let driver_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(driver_options.lex, equal_to(false));
        assert_that!(driver_options.parse, equal_to(false));
        assert_that!(driver_options.codegen, equal_to(true));
    }

    fn create_file() -> (std::path::PathBuf, TempDir) {
        let (temp, temp_dir) = temp_config_dir();
        let c_file = temp.join("HELLOWORLD.C");
        File::create(c_file.clone()).unwrap();
        (c_file, temp_dir)
    }
}
