
fn main() {
    println!("cargo:rustc-link-lib=uta");
    println!("cargo:rerun-if-changed=wrapper_header_file.h");

}
