// The shared library's soname carries the API major (design section 1.4,
// rule 6), so cargo's libclunkyard.so installs as libclunkyard.so.2.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libclunkyard.so.2");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
