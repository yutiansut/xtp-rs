use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/bridge/bridge.cpp")
        .cpp(true)
        .warnings(false)
        .flag("-std=c++11")
        .compile("bridge");

    println!("cargo:rustc-link-lib=xtptraderapi");
    println!("cargo:rustc-link-lib=xtpquoteapi");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=xtp/bin/macosx");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=xtp/bin/linux_centos7");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/wrapper.hpp");
    println!("cargo:rerun-if-changed=src/bridge/bridge.hpp");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.hpp")
        /* // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) */
        .ignore_methods()
        .rustified_enum(".*")
        .blacklist_item("XTP_SIDE_TYPE")
        .blacklist_item("XTP_POSITION_EFFECT_TYPE")
        .blacklist_item("TXTPTradeTypeType")
        .blacklist_item("TXTPOrderTypeType")
        .blacklist_function("TraderSpiStub_Rust.*")
        .blacklist_function("QuoteSpiStub_Rust.*")
        /* .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        }) */
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}