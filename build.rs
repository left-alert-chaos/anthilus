use cc::Build;

fn main() {
    println!("cargo::rerun-if-changed=./src/size/size.c");
    println!("cargo::rerun-if-changed=./src/username/uname.c");
    println!("cargo::rerun-if-changed=./src/state/io_helpers.c");
    Build::new().file("./src/size/size.c").compile("c_size");
    Build::new()
        .file("./src/username/uname.c")
        .compile("c_uname");
    Build::new()
        .file("./src/state/io_helpers.c")
        .compile("c_io");
}
