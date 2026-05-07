fn main() {
    // 1. Tell the linker not to discard 'unused' libraries. 
    // This is vital because the linker might think libc is unused 
    // before it reachs the AFL object file.
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");

    // 2. Explicitly link the dynamic loader which provides TLS symbols.
    // On most modern glibc systems, this is bundled, but AFL-RS 
    // sometimes needs the explicit DSO reference.
    println!("cargo:rustc-link-arg=-lc");

    // 3. Specifically pull in the dynamic linker to resolve the DSO error
    println!("cargo:rustc-link-arg=-Wl,--dynamic-linker=/lib64/ld-linux-x86-64.so.2");

    // 4. Restore default behavior for subsequent libraries
    println!("cargo:rustc-link-arg=-Wl,--as-needed");
}