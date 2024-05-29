extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

    use hamcrest2::prelude::*;

    use crate::{parse_command_line, validate_command_line};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
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
}
