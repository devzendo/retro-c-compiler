extern crate hamcrest2;

#[cfg(test)]
mod driver_spec {

    use std::path::PathBuf;

    use hamcrest2::prelude::*;
    use mockall::*;

    use crate::executor::MockExecutor;
    use crate::driver::{Driver, DriverOptions};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    
    #[test]
    fn empty_command() {
        let mock_executor = MockExecutor::new();
        let c_file = PathBuf::from("file.c");
        let driver_options = DriverOptions { c_file: Box::new(c_file), lex: false, parse: false, codegen: false };
        let driver = Driver::new(driver_options, Box::new(mock_executor));
        todo!();
    }

}
