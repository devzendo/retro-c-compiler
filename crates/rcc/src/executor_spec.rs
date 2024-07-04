extern crate hamcrest2;

#[cfg(test)]
mod executor_spec {

    use hamcrest2::prelude::*;

    use crate::executor::{CommandExecutor, Executor};

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn empty_command() {
        let mut e = CommandExecutor::default();
        let ex = e.run(vec![]);
        assert_that!(ex.unwrap_err().to_string(), equal_to("No command given"));
    }

    #[test]
    fn command_does_not_exist() {
        let mut e = CommandExecutor::default();
        let ex = e.run(vec!["frobnicate".to_owned()]);
        // TODO: CROSSPLATFORM
        // There will be portability differences wrt this message.
        assert_that!(
            ex.unwrap_err().to_string(),
            equal_to("Could not run command 'frobnicate': No such file or directory (os error 2)")
        );
    }

    #[test]
    fn command_exists_and_has_output() {
        // TODO: CROSSPLATFORM
        let mut e = CommandExecutor::default();
        let ex = e.run(vec!["ls".to_owned(), "Cargo.toml".to_owned()]);
        let es = ex.ok().unwrap();
        assert_that!(es.code().unwrap(), equal_to(0));
        let out = es.stdout();
        assert_that!(out, equal_to("Cargo.toml\n"));
        let err: String = es.stderr();
        assert_that!(err, equal_to(""));
    }

    #[test]
    fn command_exists_and_has_error_output() {
        // TODO: CROSSPLATFORM
        let mut e = CommandExecutor::default();
        let ex = e.run(vec!["cat".to_owned(), "nonexistant.txt".to_owned()]);
        let es = ex.ok().unwrap();
        assert_that!(es.code().unwrap(), equal_to(1));
        let out: String = es.stdout();
        assert_that!(out, equal_to(""));
        let err = es.stderr();
        assert_that!(
            err,
            equal_to("cat: nonexistant.txt: No such file or directory\n")
        );
    }
}
