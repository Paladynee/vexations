pub mod lexer_test_generator;

use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum GeneratorMode {
    GenLexerTest(usize),
}

pub fn generate(out_file: PathBuf, generator_mode: GeneratorMode) {
    match generator_mode {
        GeneratorMode::GenLexerTest(n_tok) => {
            let file = File::create(out_file).unwrap();
            let mut writer = std::io::BufWriter::new(file);
            lexer_test_generator::generate_lexer_test(&mut writer, n_tok)
                .unwrap();
        }
    };
}
