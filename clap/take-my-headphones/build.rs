fn main() {
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR env var"));

    let clap_bindings = bindgen::Builder::default()
        .header("../../external/clap-1.2.7/include/clap/clap.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        //.generate_cstr(true)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Unsigned)
        .generate()
        .expect("clap bindgen failed");

    clap_bindings.write_to_file(out.join("clap.rs")).expect("write failed");
}
