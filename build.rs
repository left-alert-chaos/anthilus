use cc;

fn main() {
    println!("cargo::rerun-if-changed=./src/size/size.c");
    println!("cargo::rerun-if-changed=./src/username/uname.c");
    cc::Build::new().file("./src/size/size.c").compile("c_size");
    cc::Build::new().file("./src/username/uname.c").compile("c_uname");
}
