use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("cargo:rustc-link-search=k4a-orbbec-sdk/windows-desktop/amd64/release/lib");
    println!("cargo:rustc-link-search={}/k4a-orbbec-sdk/windows-desktop/amd64/release/lib", src_dir);
    println!("cargo:rustc-link-lib=dylib=k4a");
    println!("cargo:rustc-link-lib=k4arecord");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&[
            "-I./k4a-orbbec-sdk/include"
        ])
        .rustified_enum("[kK]4[aA].*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");

    let bin_path = PathBuf::from("k4a-orbbec-sdk/windows-desktop/amd64/release/bin");

    let dlls = ["k4a.dll", "k4arecord.dll", "live555.dll", "ob_usb.dll", "OrbbecSDK.dll"];
    for dll in dlls {
        fs::copy(bin_path.join(dll), out_path.join(dll)).expect("Failed to copy dll");
    }
}
