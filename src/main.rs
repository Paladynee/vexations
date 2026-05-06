#![feature(likely_unlikely)]

use std::path::PathBuf;

use vexations::GeneratorMode;
use vexations::Options;
use vexations::RunningMode;
use vexations::compiler;
use vexations::generator;

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
        RunningMode::Compile => compiler::compile(options),
        RunningMode::Generate => generator::generate(options),
    }
}
