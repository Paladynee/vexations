use std::path::PathBuf;

use vexations_compiler::compiler;
use vexations_compiler::frontend::token::TokenKind;
use vexations_generator::GeneratorMode;

#[derive(Debug, Clone, Copy)]
enum RunningMode {
    Compile,
    Generate,
}

#[derive(Debug, Clone, Default)]
struct Options {
    in_files: Vec<PathBuf>,
    out_file: Option<PathBuf>,
    generator_mode: Option<GeneratorMode>,
}

fn main() {
    let mut args = std::env::args_os().skip(1);
    let mut options = Options::default();
    let mut mode = RunningMode::Compile;

    let mut errored = false;
    while let Some(arg) = args.next() {
        match arg.as_encoded_bytes() {
            b"compile" => mode = RunningMode::Compile,
            b"generate" => mode = RunningMode::Generate,

            b"-l" => {
                let Some(tokens_to_gen) = args.next() else {
                    eprintln!(
                        "Expected number of tokens to generate after {}",
                        arg.display()
                    );
                    errored = true;
                    continue;
                };
                let Some(parsed) = tokens_to_gen
                    .to_str()
                    .and_then(|s| s.parse::<usize>().ok())
                else {
                    eprintln!(
                        "Expected a valid number after {}, got {}",
                        arg.display(),
                        tokens_to_gen.to_string_lossy()
                    );
                    errored = true;
                    continue;
                };
                options.generator_mode =
                    Some(GeneratorMode::GenLexerTest(parsed));
            }

            b"-i" | b"--input" => {
                let Some(next_arg) = args.next() else {
                    eprintln!("Expected input file after {}", arg.display());
                    errored = true;
                    continue;
                };
                options.in_files.push(PathBuf::from(next_arg));
            }
            b"-o" | b"--output" => {
                let Some(next_arg) = args.next() else {
                    eprintln!("Expected output file after {}", arg.display());
                    errored = true;
                    continue;
                };
                options.out_file = Some(PathBuf::from(next_arg));
            }
            _ => {
                eprintln!("Unknown argument: {}", arg.display());
                errored = true;
            }
        }
    }

    if errored {
        return;
    }

    match mode {
        RunningMode::Compile => {
            if options.in_files.is_empty() {
                eprintln!(
                    "No input files provided for compilation. Use -i <file> to specify input files."
                );
                return;
            };
            let Some(out_file) = options.out_file else {
                eprintln!(
                    "Output file not specified for compilation. Use -o <file> to specify the output file for compiled code."
                );
                return;
            };
            compiler::compile(options.in_files, out_file);
        }
        RunningMode::Generate => {
            let Some(generator_mode) = options.generator_mode else {
                eprintln!(
                    "Generator mode not specified. Use -l <number> to specify the number of tokens to generate for lexer tests."
                );
                return;
            };
            let Some(out_file) = options.out_file else {
                eprintln!(
                    "Output file not specified. Use -o <file> to specify the output file for generated code."
                );
                return;
            };
            vexations_generator::generate(out_file, generator_mode);
        }
    }
}
