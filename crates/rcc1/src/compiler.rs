use std::path::PathBuf;

use common::target_platform::TargetPlatform;

#[derive(Debug, Clone)]
pub struct CompilerOptions {
    pub c_file: Box<PathBuf>,
    pub lex: bool,
    pub parse: bool,
    pub codegen: bool,
    pub target_platform: TargetPlatform,
}
