fn main() {
    println!("cargo:rustc-link-lib=lxc");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let out_path = std::path::PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
