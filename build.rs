fn main() {
    cc::Build::new().file("src/wrappers.c").compile("wrappers");
}
