fn main() {
    #[cfg(debug_assertions)]
    println!("cargo:rustc-env=RUST_LOG=debug");
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-env=RUST_LOG=info");
}
