use std::path::PathBuf;

pub struct SuffixTranslator {
    c_file: PathBuf,
}

impl SuffixTranslator {
    fn new(c_file: PathBuf) -> Self {
        Self { c_file }
    }

    fn preprocessor(&self) -> PathBuf {
        let mut out = self.c_file.clone();
        out.set_extension("i");
        out
    }
}

#[cfg(test)]
#[path = "./suffix_translator_spec.rs"]
mod suffix_translator_spec;
