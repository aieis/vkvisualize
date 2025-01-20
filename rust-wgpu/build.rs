fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let manifest_dir_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = std::env::var("PROFILE").unwrap();
    let path = std::path::Path::new(&manifest_dir_string).join("target").join(build_type);
    let out_path = std::path::PathBuf::from(path);
    let bin_path = std::path::PathBuf::from("third_party/k4a-orbbec-sys/k4a-orbbec-sdk/windows-desktop/amd64/release/bin");

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
