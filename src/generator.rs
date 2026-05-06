mod lexer_test_generator;

use std::fs::File;

use crate::GeneratorMode;
use crate::Options;

pub fn generate(
    Options {
        out_file,
        generator_mode,
        ..
    }: Options,
) {
    let Some(mode) = generator_mode else {
        eprintln!("No generator mode specified");
        return;
    };
    let Some(out_file) = out_file else {
        eprintln!("No output file specified");
        return;
    };

    match mode {
        GeneratorMode::GenLexerTest(n_tok) => {
            let file = File::create(out_file).unwrap();
            let mut writer = std::io::BufWriter::new(file);
            lexer_test_generator::generate_lexer_test(&mut writer, n_tok)
                .unwrap();
        }
    };
}
