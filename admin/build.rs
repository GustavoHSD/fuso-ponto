fn main() {
    println!("cargo:rerun-if-env-changed=RAVEN_LIB_DIR");
    // rpath so the built binary finds libraven.so without LD_LIBRARY_PATH
    if let Ok(dir) = std::env::var("RAVEN_LIB_DIR") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{dir}");
    }
}
