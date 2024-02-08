fn main() {
    cc::Build::new()
        .file("src/myc.c")
        .compile("myc");
    println!("cargo:rerun-if-changed=src/myc.c");
}
