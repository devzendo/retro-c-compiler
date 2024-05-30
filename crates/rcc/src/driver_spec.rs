extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

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

    
}
