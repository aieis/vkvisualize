use std::env;
use std::path::{PathBuf, Path};
use std::fs;

fn main() {
    let out_path = get_output_path();
    let bin_path = PathBuf::from("third_party/k4a-orbbec-sys/k4a-orbbec-sdk/windows-desktop/amd64/release/bin");

    let dlls = ["k4a.dll", "k4arecord.dll", "live555.dll", "ob_usb.dll", "OrbbecSDK.dll", "depthengine_2_0.dll"];
    for dll in dlls {
        fs::copy(bin_path.join(dll), out_path.join(dll)).expect("Failed to copy dll");
    }

}

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}
