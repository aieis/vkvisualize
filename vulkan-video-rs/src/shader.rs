use std::{
    path::{Path, PathBuf}, process::ExitStatus, time::SystemTime
};

use ash::vk;
use comptime_register_macro::register_shader;

use crate::vk_bundles::DeviceBundle;

fn compile_shader_modules(
    device: &ash::Device,
    vertex_code: &[u8],
    fragment_code: &[u8],
) -> (vk::ShaderModule, vk::ShaderModule) {
    let create_info = vk::ShaderModuleCreateInfo {
        code_size: vertex_code.len(),
        p_code: vertex_code.as_ptr() as *const u32,
        ..Default::default()
    };

    let vertex = unsafe { device.create_shader_module(&create_info, None).unwrap() };

    let create_info = vk::ShaderModuleCreateInfo {
        code_size: fragment_code.len(),
        p_code: fragment_code.as_ptr() as *const u32,
        ..Default::default()
    };

    let fragment = unsafe { device.create_shader_module(&create_info, None).unwrap() };

    (vertex, fragment)
}

pub struct ShaderComp {
    pub vertex_code: Vec<u8>,
    pub fragment_code: Vec<u8>,
}

pub struct ShaderFile {
    pub vertex_path: String,
    pub fragment_path: String,
}

pub trait Shader {
    fn compile(&self, device: &ash::Device) -> (vk::ShaderModule, vk::ShaderModule);
}

impl Shader for ShaderFile {
    fn compile(&self, device: &ash::Device) -> (vk::ShaderModule, vk::ShaderModule) {
        let vertex_code = std::fs::read(&self.vertex_path).unwrap();
        let fragment_code = std::fs::read(&self.fragment_path).unwrap();
        return compile_shader_modules(device, &vertex_code, &fragment_code);
    }
}

impl Shader for ShaderComp {
    fn compile(&self, device: &ash::Device) -> (vk::ShaderModule, vk::ShaderModule) {
        return compile_shader_modules(device, &self.vertex_code, &self.fragment_code);
    }
}

#[macro_export]
macro_rules! make_shader {
    ($x: literal) => {{
        let vertex_code = include_bytes!(concat!("../assets/shaders/", $x, ".vert.spv")).to_vec();
        let fragment_code = include_bytes!(concat!("../assets/shaders/", $x, ".frag.spv")).to_vec();
        ShaderComp {
            vertex_code,
            fragment_code,
        }
    }};
}

#[register_shader("mesh")]
pub struct ShaderMesh {}

#[register_shader("triangle")]
pub struct ShaderRect {}

#[register_shader("texture")]
pub struct ShaderTexture {}

pub struct StaticShader {
    pub vert_path: PathBuf,
    pub frag_path: PathBuf,

    pub vert_mt: SystemTime,
    pub frag_mt: SystemTime,

    pub vert_spv_path: PathBuf,
    pub frag_spv_path: PathBuf,

    pub vert_code: Vec<u8>,
    pub frag_code: Vec<u8>,

    pub id: usize,
}

pub struct CompiledSahder {
    pub details: StaticShader,
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
}

impl StaticShader {
    pub fn reload_and_compile(&mut self, device: &DeviceBundle) -> bool {
        let ve = std::fs::exists(&self.vert_path);
        let ve = if let Ok(v) = ve { v } else { false };

        let fe = std::fs::exists(&self.frag_path);
        let fe = if let Ok(v) = fe { v } else { false };

        if !ve {
            println!("Error: File '{:?}' does not exist!", self.vert_path);
        }
        if !fe {
            println!("Error: File '{:?}' does not exist!", self.frag_path);
        }

        if !ve || !fe {
            return false;
        }

        // let vert_result = StaticShader::reload_and_compile_shader(device, )

        // recompile
        return true;
    }

    pub fn reload_and_compile_shader(device: &DeviceBundle, glsl_path: &PathBuf, spv_path: &PathBuf) -> Option<(Vec<u8>, vk::ShaderModule, SystemTime)> {

        let mt = match StaticShader::file_mt(&glsl_path) {
            Some(mt) => mt,
            None => return None
        };


        if !StaticShader::generate_spv(glsl_path, spv_path) {
            println!("Error: Failed to generate the spv file for shader: {:?}.", glsl_path);
            return None;
        }

        let code = match StaticShader::read_file(spv_path) {
            Some(code) => code,
            None => {
                println!("Error: Failed to read the spv file for shader: {:?}.", spv_path);
                return None;
            }
        };

        let create_info = vk::ShaderModuleCreateInfo {
            code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
            ..Default::default()
        };

        let module = unsafe { device.logical.create_shader_module(&create_info, None).unwrap() };
        return Some((code, module, mt));
    }

    pub fn outdated(&self) -> bool {
        let mut err = false;
        let mut new = false;

        if let Some(mt) = StaticShader::file_mt(&self.vert_path) {
            new = new || mt != self.vert_mt;
        } else {
            err = true;
        }

        if let Some(mt) = StaticShader::file_mt(&self.frag_path) {
            new = new || mt != self.frag_mt;
        } else {
            err = true;
        }

        return new && !err;
    }

    fn read_file(file: &PathBuf) -> Option<Vec<u8>> {
        match std::fs::read(file) {
            Ok(data) => Some(data),
            Err(e) => {
                println!(
                    "Error: Failed to read bytes from file: '{:?}: \n\t {}'",
                    file, e
                );
                None
            }
        }
    }

    fn file_mt(file: &PathBuf) -> Option<SystemTime> {
        match std::fs::metadata(file) {
            Ok(res) => match res.modified() {
                Ok(mt) => Some(mt),

                Err(e) => {
                    println!(
                        "Failed to get modified time from path: '{:?}' \n\t {}",
                        file, e
                    );
                    None
                }
            },
            Err(e) => {
                println!("Failed to get metadata from path: '{:?}' \n\t {}", file, e);
                None
            }
        }
    }

    fn generate_spv(src: &PathBuf, dst: &PathBuf) -> bool {
        let mut cmd = std::process::Command::new("glslc");
        cmd.arg(src)
            .arg("-o")
            .arg(dst);

        match cmd.output() {
            Ok(res) => {

                if res.status.success() { true } else {
                    println!("Error: Spv command failed with exit code: {:?}", res.status.code());
                    false
                }


            }
            Err(e) => {
                println!("Error: Spv command failed to execute. \n Cmd: \n\t {:?} \n Err: \n\t {}.", cmd, e);
                false
            }
        }
    }
}
