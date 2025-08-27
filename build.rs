fn main() {
    println!("cargo:rerun-if-changed=script/memory.ld");
    println!("cargo:rerun-if-changed=build.rs");
}
