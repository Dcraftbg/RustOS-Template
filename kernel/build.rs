fn main() {
    println!("cargo:rustc-link-arg=-Tlinker/link.ld");
    println!("cargo:rerun-if-changed=linker/link.ld");
}
