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
    fn assembler() {
        let c_file = PathBuf::from("file.c");
        let xlat = SuffixTranslator::new(c_file);
        assert_that!(xlat.assembler(), equal_to(PathBuf::from("file.asm")));
    }

    #[test]
    fn binary() {
        let c_file = PathBuf::from("file.c");
        let xlat = SuffixTranslator::new(c_file);
        assert_that!(xlat.binary(), equal_to(PathBuf::from("file.bin")));
    }

    #[test]
    fn listing() {
        let c_file = PathBuf::from("file.c");
        let xlat = SuffixTranslator::new(c_file);
        assert_that!(xlat.listing(), equal_to(PathBuf::from("file.lst")));
    }
}
