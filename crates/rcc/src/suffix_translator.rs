use std::path::PathBuf;

pub struct SuffixTranslator {
    c_file: PathBuf,
}

impl SuffixTranslator {
    pub fn new(c_file: PathBuf) -> Self {
        Self { c_file }
    }

    pub fn preprocessor(&self) -> PathBuf {
        let mut out = self.c_file.clone();
        out.set_extension("i");
        out
    }

    // TODO The output of the compiler is an assembler file, not an object. 
    // rename this to assembler
    pub fn compiler(&self) -> PathBuf {
        let mut out = self.c_file.clone();
        out.set_extension("o");
        out
    }

    pub fn binary(&self) -> PathBuf {
        let mut out = self.c_file.clone();
        out.set_extension("bin");
        out
    }

    pub fn listing(&self) -> PathBuf {
        let mut out = self.c_file.clone();
        out.set_extension("lst");
        out
    }
}

#[cfg(test)]
#[path = "./suffix_translator_spec.rs"]
mod suffix_translator_spec;
