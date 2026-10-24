use cc;

fn main() {
    println!("cargo::rerun-if-changed=src/username/username.c");
    cc::Build::new()
        .file("src/username/username.c")
        .compile("c_username");
}
