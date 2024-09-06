extern crate hamcrest2;

#[cfg(test)]
mod command_line_spec {

    use std::{fs::File, path::PathBuf};

    use common::target_platform::TargetPlatform;
    use common_test::file_utils_test_helper::temp_config_dir;
    use hamcrest2::prelude::*;
    use temp_testdir::TempDir;

    use crate::command_line::{parse_command_line, validate_command_line};

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
    fn not_an_i_file_given() {
        let arg_vec = vec!["rcc1", "aardvark.stl"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'aardvark.stl' is not a preprocessed C filename (.i)")
        );
    }

    #[test]
    fn nonexistant_lowercase_i_file_given() {
        let arg_vec = vec!["rcc1", "magnumopus.i"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'magnumopus.i' could not be found")
        );
    }

    #[test]
    fn nonexistant_uppercase_i_file_given() {
        let arg_vec = vec!["rcc1", "HELLOWORLD.I"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'HELLOWORLD.I' could not be found")
        );
    }

    #[test]
    fn existing_uppercase_i_file_given() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap()];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(&result, ok());
        assert_that!(
            result.unwrap().c_file.to_str().unwrap(),
            equal_to(i_file.to_str().unwrap())
        );
    }

    #[test]
    fn not_an_asm_file_given_lowercase() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-o", "booplesnoot.pdf"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'booplesnoot.pdf' is not an assembler file (.asm)")
        );
    }

    #[test]
    fn not_an_asm_file_given_uppercase() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-o", "ZEITGEIST.SVG"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap());
        assert_that!(
            result.unwrap_err().to_string(),
            equal_to("'ZEITGEIST.SVG' is not an assembler file (.asm)")
        );
    }

    #[test]
    fn an_asm_file_given_lowercase() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-o", "output.asm"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap()).expect("Expected a valid command line");
        assert_that!(result.asm_file, eq(Some(Box::new(PathBuf::from("output.asm".to_owned())))));
    }

    #[test]
    fn an_asm_file_given_uppercase() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-o", "OUTPUT.ASM"];
        let result = validate_command_line(parse_command_line(arg_vec).unwrap()).expect("Expected a valid command line");
        assert_that!(result.asm_file, eq(Some(Box::new(PathBuf::from("OUTPUT.ASM".to_owned())))));
    }

    #[test]
    fn all_flags_off_by_default() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap()];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.lex, equal_to(false));
        assert_that!(compiler_options.parse, equal_to(false));
        assert_that!(compiler_options.codegen, equal_to(false));
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::Transputer));
    }

    #[test]
    fn lex_flag() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "--lex"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.lex, equal_to(true));
    }

    #[test]
    fn parse_flag() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "--parse"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.parse, equal_to(true));
    }

    #[test]
    fn codegen_flag() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "--codegen"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.codegen, equal_to(true));
    }

    #[test]
    fn architecture_epoc16() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-a", "EPOC16"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::EPOC16));
    }

    #[test]
    fn architecture_x865_64() {
        let (i_file, _temp_dir) = create_file();

        let arg_vec = vec!["rcc1", i_file.to_str().unwrap(), "-a", "X86_64"];
        let compiler_options = validate_command_line(parse_command_line(arg_vec).unwrap()).unwrap();
        assert_that!(compiler_options.target_platform, equal_to(TargetPlatform::X86_64));
    }

    fn create_file() -> (PathBuf, TempDir) {
        let (temp, temp_dir) = temp_config_dir();
        let i_file = temp.join("HELLOWORLD.I");
        File::create(i_file.clone()).unwrap();
        (i_file, temp_dir)
    }
}

