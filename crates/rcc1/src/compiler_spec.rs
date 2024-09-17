extern crate hamcrest2;

#[cfg(test)]
mod compiler_spec {
    use anyhow::{Error, Result};
    use common::target_platform::TargetPlatform;
    use common_test::file_utils_test_helper::temp_config_dir;
    use hamcrest2::prelude::*;
    use std::io::Write;
    use std::fs::File;
    use sysexits::ExitCode;

    use crate::compiler::{Compiler, CompilerOptions};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn just_lexer_test_ok_listing_1_1() {
        // no preprocessor changes in this simple code, so .c not .i
        let contents = include_str!("listing_1_1.c").as_ref();
        let out = lexer_test(contents);
        assert!(out.is_ok());
        assert_that!(out.unwrap(), eq(ExitCode::Ok));
    }

    #[test]
    fn just_lexer_test_fail() {
        // no preprocessor changes in this simple code, so .c not .i
        let contents = "<<>>".as_bytes();
        let out = lexer_test(contents);
        assert!(out.is_ok()); // an Err is a failure
        assert_that!(out.unwrap(), eq(ExitCode::DataErr));
    }

    fn lexer_test(contents: &[u8]) -> Result<ExitCode, Error> {
        let (temp, _temp_dir) = temp_config_dir();
        let i_file = temp.join("file.i");
        let mut file = File::create(i_file.clone())?;
        file.write(contents).expect("Expected to write file contents");
        drop(file);

        let compiler_options = CompilerOptions {
            c_file: Box::new(i_file.clone()),
            asm_file: None,
            lex: true,
            parse: false,
            codegen: false,
            target_platform: TargetPlatform::default(),
        };
        let compiler = Compiler::new();
        compiler.compile(compiler_options)
    }
}
