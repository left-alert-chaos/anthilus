use cc;

fn main() {
    println!("cargo::rerun-if-changed=src/size/size.c");
    println!("cargo::rerun-if-changed=src/runtime/io_helpers.c");
    println!("cargo::rerun-if-changed=src/username/username.c");
    cc::Build::new().file("src/size/size.c").compile("c_size");
    cc::Build::new()
        .file("src/runtime/io_helpers.c")
        .compile("c_raw_mode");
    cc::Build::new()
        .file("src/username/username.c")
        .compile("c_username");
}
