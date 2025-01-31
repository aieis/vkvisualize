fn main() {    
    let cargo_manifest_dir= std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-search={}/k4a-orbbec-sdk/windows-desktop/amd64/release/lib", cargo_manifest_dir);
    println!("cargo:rustc-link-lib=dylib=k4a");
    println!("cargo:rustc-link-lib=k4arecord");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&[
            "-I./k4a-orbbec-sdk/include"
        ])
        .rustified_enum("[kK]4[aA].*")
        .generate()
        .expect("Unable to generate bindings");

    let bindings_dir = std::path::Path::new(&cargo_manifest_dir).join("bindings");
    let bindings_file = bindings_dir.join("bindings.rs");

    if let Err(e) = std::fs::create_dir_all(&bindings_dir) {
        panic!("failed to create directory {}: {}", bindings_dir.display(), e);
    }

    bindings
        .write_to_file(bindings_file)
        .expect("Unable to write bindings");

    let bin_path = std::path::PathBuf::from("k4a-orbbec-sdk/windows-desktop/amd64/release/bin");

    let build_type = std::env::var("PROFILE").unwrap();
    let path = std::path::Path::new(&cargo_manifest_dir).join("target").join(build_type);
    let out_path = std::path::PathBuf::from(path);
    
    let dlls = ["k4a.dll", "k4arecord.dll", "live555.dll", "ob_usb.dll", "OrbbecSDK.dll", "depthengine_2_0.dll"];
    for dll in dlls {
        let target = out_path.join(dll);
        let src = bin_path.join(dll);
        if let Ok(m) = unchanged(&src, &target) {
            if m {
                continue;
            }
        }

        std::fs::copy(src, target).expect("Failed to copy dll");
    }
    
}

fn unchanged(src: &std::path::PathBuf, target: &std::path::PathBuf) -> std::io::Result<bool> {
    let m1 = std::fs::metadata(&target)?;
    let m2 = std::fs::metadata(&src)?;
    Ok(m1.modified()? == m2.modified()?)
}
