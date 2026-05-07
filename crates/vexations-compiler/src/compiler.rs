use std::fs;
use std::path::PathBuf;

use printerator::PrinterateDisplay;

use crate::compiler::lexer::lex;
use crate::frontend::source::VexationsSource;

pub mod lexer;

pub fn compile(in_files: Vec<PathBuf>, out_file: PathBuf) {
    if in_files.is_empty() {
        eprintln!("No input files provided");
        return;
    }

    for in_file in in_files {
        let mut bytes = fs::read(&in_file).unwrap();
        bytes.extend_from_slice(&[0; 3]);
        let source = VexationsSource::try_from_bytes(&bytes).unwrap();

        let (tokens, spans, idents, errors) = lex(source.clone()).finalize();

        if !errors.is_empty() {
            eprintln!("errors during lexing:");
            for error in errors {
                eprintln!(
                    "in file {}\n\t{}",
                    in_file.display(),
                    error.display(source.clone())
                );
            }
            continue;
        }

        println!("lex finished successfully");
        println!(
            "Tokens: {}",
            tokens
                .get(0..(tokens.len().min(10)))
                .unwrap_or_default()
                .iter()
                .map(|t| t.source_repr())
                .printer()
        );
        println!(
            "Idents: {}",
            idents
                .get(0..(idents.len().min(10)))
                .unwrap_or_default()
                .iter()
                .printer()
        );
    }
    todo!()
}
