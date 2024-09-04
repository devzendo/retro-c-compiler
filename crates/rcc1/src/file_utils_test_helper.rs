
use hamcrest2::prelude::*;
use std::path::Path;
use temp_testdir::TempDir;

// Code in this module is used by tests.

#[allow(dead_code)]
pub fn temp_config_dir() -> (Box<Path>, TempDir) {
    // Return both objects as if temp_dir is not moved back to the caller, it'll drop and
    // delete.

    let temp_dir = TempDir::default();
    let temp = temp_dir.to_path_buf();
    assert_that!(temp.as_path(), dir_exists());

    (temp.into_boxed_path(), temp_dir)
}
