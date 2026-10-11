use cc;

fn main() {
    println!("cargo::rerun-if-changed=src/size/size.c");
    cc::Build::new()
        .file("src/size/size.c")
        .compile("c_size");
}
