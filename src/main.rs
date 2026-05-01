#![feature(likely_unlikely)]

pub mod lexer;
pub mod source;
pub mod token;

use std::env;
use std::fs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::source::VexationsSource;

struct XorShift32(u32);
impl XorShift32 {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        Self(if seed == 0 { 1 } else { seed })
    }

    #[inline(always)]
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: vexations <subcommand> [args...]");
        eprintln!("Subcommands:");
        eprintln!(
            "  lex <file>                       - Lex a source file and benchmark it"
        );
        eprintln!(
            "  gensrc <mode> <output_file>      - Generate a stress-test source file"
        );
        return;
    }

    match args[1].as_str() {
        "lex" => {
            if args.len() < 3 {
                eprintln!("Usage: vexations lex <file>");
                return;
            }
            run_lex(&args[2]);
        }
        "gensrc" => {
            if args.len() < 5 {
                eprintln!("Usage: vexations gensrc <mode> <output_file> <len>");
                eprintln!("Available modes: angles");
                return;
            }
            run_gensrc(
                &args[2],
                &args[3],
                args[4].parse().expect("enter a damn number"),
            );
        }
        _ => {
            eprintln!("Unknown subcommand: {}", args[1]);
        }
    }
}

fn run_gensrc(mode: &str, out_path: &str, len: usize) {
    match mode {
        "angles" => {
            let mut rng = XorShift32::new();

            let tokens: &[&[u8]] = &[b"<", b"<=", b"<<", b"<<="];
            let spaces: &[&[u8]] =
                &[b" ", b"  ", b"\t", b"\n", b" \n  ", b"\n\n\t"];

            let target_size = len;
            let mut buf = Vec::with_capacity(target_size + 100);

            while buf.len() < target_size {
                let space_idx = (rng.next() as usize) % spaces.len();
                let token_idx = (rng.next() as usize) % tokens.len();

                buf.extend_from_slice(spaces[space_idx]);
                buf.extend_from_slice(tokens[token_idx]);
            }

            fs::write(out_path, buf).expect("failed to write generated source");
            println!(
                "Generated {} bytes of angle-bracket stress test to '{}'",
                target_size, out_path
            );
        }
        _ => {
            eprintln!("Unknown mode '{}'. Available modes: angles", mode);
        }
    }
}

fn run_lex(file_path: &str) {
    // very primitive module loader here:
    let mut src = fs::read(file_path).unwrap();
    // source extender with 3 zero bytes for performant lexer,
    // should be handled in module loader
    src.extend_from_slice(&[0, 0, 0]);
    let Some(source) = VexationsSource::try_from_bytes(src.as_slice()) else {
        eprintln!("file {} is not ascii source!", file_path);
        return;
    };

    let times = 100000000 / src.len().max(1);
    eprintln!(
        "running the lexer on output of size {}, running {} times",
        src.len(),
        times
    );

    let start = std::time::Instant::now();
    let mut tokens = vec![];
    let mut errors = vec![];
    let mut idents = vec![];

    for _ in 0..times {
        tokens.clear();
        errors.clear();
        idents.clear();
        lexer::lex(&source, &mut tokens, &mut errors, &mut idents);
    }

    let end = start.elapsed();
    eprintln!("took {:?} to lex the same program {} times", end, times);
    let bytes_per_sec = (src.len() as f64 * times as f64) / end.as_secs_f64();
    eprintln!("that's {:.2} bytes/sec", bytes_per_sec);

    println!(
        "tokens len: {}, tokens: {:#?}",
        tokens.len(),
        tokens.get(0..(10.min(tokens.len()))).unwrap_or_default()
    );
    println!(
        "idents len: {}, idents: {:#?}",
        idents.len(),
        idents.get(0..(10.min(idents.len()))).unwrap_or_default()
    );
    println!(
        "errors len: {}, errors: {:#?}",
        errors.len(),
        errors.get(0..(10.min(errors.len()))).unwrap_or_default()
    );
}
