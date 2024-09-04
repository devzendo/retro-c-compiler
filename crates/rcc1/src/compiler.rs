use std::path::PathBuf;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetPlatform {
    #[default]
    Transputer,
    EPOC16,
    X86_64,
}

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
    pub target_platform: TargetPlatform,
}
