extern crate hamcrest2;

#[cfg(test)]
mod suffix_translator_spec {

    use std::path::PathBuf;

    use hamcrest2::prelude::*;
    use crate::suffix_translator::SuffixTranslator;

    #[ctor::ctor]
    fn before_each() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn preprocessor() {
        let c_file = PathBuf::from("file.c");
        let xlat = SuffixTranslator::new(c_file);
        assert_that!(xlat.preprocessor(), equal_to(PathBuf::from("file.i")));
    }

    #[test]
    fn compiler() {
        let c_file = PathBuf::from("file.c");
        let xlat = SuffixTranslator::new(c_file);
        assert_that!(xlat.compiler(), equal_to(PathBuf::from("file.o")));
    }
}
