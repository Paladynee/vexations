#![feature(likely_unlikely)]

use std::path::PathBuf;

pub mod compiler;
pub mod generator;
pub mod middle;

#[derive(Debug, Clone, Copy)]
pub enum RunningMode {
    Compile,
    Generate,
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    pub in_files: Vec<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub generator_mode: Option<GeneratorMode>,
}

#[derive(Debug, Clone)]
pub enum GeneratorMode {
    GenLexerTest(usize),
}
