fn main() {
    cc::Build::new().file("src/wrappers.c").compile("wrappers");
    println!("cargo:rerun-if-changed=src");
}
