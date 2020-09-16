extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake;

fn main() {


    // Builds the azure iot sdk, installing it
    // into $OUT_DIR
    use cmake::Config;

    let dst = Config::new("azure-iot-sdk-c")
                     .define("use_edge_modules", "ON")
                     .build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    
    println!("cargo:rustc-link-search=native={}", dst.display());

    // Tell cargo to tell rustc to link the azureiot libraries.
    println!("cargo:rustc-link-lib=iothub_client_mqtt_transport");
    println!("cargo:rustc-link-lib=iothub_client");
    println!("cargo:rustc-link-lib=parson");
    println!("cargo:rustc-link-lib=umqtt");
    println!("cargo:rustc-link-lib=prov_auth_client");
    println!("cargo:rustc-link-lib=hsm_security_client");
    println!("cargo:rustc-link-lib=uhttp");
    println!("cargo:rustc-link-lib=aziotsharedutil");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=uuid");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // additional clang arguments.
        .clang_arg(format!("-I{}/include/azureiot", dst.display()))
        .clang_arg("-DUSE_EDGE_MODULES")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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