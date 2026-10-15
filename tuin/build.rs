use cc;

fn main() {
    println!("cargo::rerun-if-changed=src/size/size.c");
    println!("cargo::rerun-if-changed=src/runtime/raw_mode.c");
    cc::Build::new().file("src/size/size.c").compile("c_size");
    cc::Build::new()
        .file("src/runtime/raw_mode.c")
        .compile("c_raw_mode");
}
