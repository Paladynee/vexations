#![no_main]
use libfuzzer_sys::fuzz_target;
use vexations::compiler::lexer::lex;
use vexations::middle::source::VexationsSource;

fuzz_target!(|data: &[u8]| {
    // The VexationsSource requires 3 zero bytes at the end for padding.
    // We need to ensure the input is valid ASCII and has proper padding.
    
    // Skip if input doesn't contain valid ASCII
    if !data.is_ascii() {
        return;
    }
    
    // Create a buffer with 3 zero bytes of padding at the end
    let mut padded = data.to_vec();
    padded.extend_from_slice(&[0u8; 3]);
    
    // Try to create a VexationsSource from the padded buffer
    if let Some(src) = VexationsSource::try_from_bytes(&padded) {
        // Fuzz the lexer with this input
        let (tokens, idents, errors) = lex(src);
        
        // We're not asserting anything specific here - we're primarily checking
        // that the lexer doesn't panic or crash on arbitrary input.
        // The fuzzer will track coverage and look for crashes automatically.
        let _ = (tokens, idents, errors);
    }
});
