use std::{
    path::{Path, PathBuf}, process::ExitStatus, str::FromStr, time::SystemTime
};

use anyhow::Error;
use ash::vk;
use comptime_register_macro::{register_shader, shaders_registry};

use crate::{drawable::drawable_common::{DescSetBinding, PipelineDescriptor}, vk_bundles::DeviceBundle};

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
pub struct ShaderMesh { }

impl ShaderMesh  {
    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

#[register_shader("triangle")]
pub struct ShaderRect {}
impl ShaderRect {
    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

#[register_shader("texture")]
pub struct ShaderTexture {}

impl ShaderTexture {
    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![
            DescSetBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
            }
        ];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}

pub struct StaticShader {
    pub vert_path: PathBuf,
    pub frag_path: PathBuf,

    pub vert_mt: SystemTime,
    pub frag_mt: SystemTime,

    pub vert_spv_path: PathBuf,
    pub frag_spv_path: PathBuf,

    pub vert_code: Vec<u8>,
    pub frag_code: Vec<u8>,

    pub descriptor: PipelineDescriptor,

    pub id: usize,
}

pub struct CompiledShader {
    pub details: StaticShader,
    pub vert_module: vk::ShaderModule,
    pub frag_module: vk::ShaderModule,
}

impl StaticShader {

    pub fn reload_and_compile_glsl(device: &DeviceBundle, glsl_path: &PathBuf, spv_path: &PathBuf) -> Option<(Vec<u8>, vk::ShaderModule, SystemTime)> {

        let mt = match StaticShader::file_mt(&glsl_path) {
            Some(mt) => mt,
            None => return None
        };


        if !StaticShader::generate_spv(glsl_path, spv_path) {
            println!("Error: Failed to generate the spv file for shader: {:?}.", glsl_path);
            return None;
        }

        let (code, module) = match StaticShader::reload_and_compile_spv(device, spv_path) {
            Some((code, module)) => (code, module),
            None => return None
        };

        return Some((code, module, mt));
    }

    pub fn reload_and_compile_spv(device: &DeviceBundle, spv_path: &PathBuf) -> Option<(Vec<u8>, vk::ShaderModule)> {
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
        return Some((code, module));
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

    pub fn changed(file: &PathBuf, file_mt: SystemTime) -> bool {
        if let Some(mt) = StaticShader::file_mt(&file) {
            return mt != file_mt;
        }

        return false;
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

impl CompiledShader {
    pub fn reload_and_compile(&mut self, device: &DeviceBundle) -> bool {

        let ve = std::fs::exists(&self.details.vert_path);
        let ve = if let Ok(v) = ve { v } else { false };

        let fe = std::fs::exists(&self.details.frag_path);
        let fe = if let Ok(v) = fe { v } else { false };

        if !ve {
            println!("Error: File '{:?}' does not exist!", self.details.vert_path);
        }
        if !fe {
            println!("Error: File '{:?}' does not exist!", self.details.frag_path);
        }

        if !ve || !fe {
            return false;
        }

        if StaticShader::changed(&self.details.vert_path, self.details.vert_mt) {

            if let Some((code, module, mt)) = StaticShader::reload_and_compile_glsl(device, &self.details.vert_path, &self.details.vert_spv_path) {
                unsafe { device.logical.destroy_shader_module(self.vert_module, None) };
                self.vert_module = module;
                self.details.vert_code = code;
                self.details.vert_mt = mt;
            }

        }


        if StaticShader::changed(&self.details.frag_path, self.details.frag_mt) {

            if let Some((code, module, mt)) = StaticShader::reload_and_compile_glsl(device, &self.details.frag_path, &self.details.frag_spv_path) {
                unsafe { device.logical.destroy_shader_module(self.frag_module, None) };
                self.frag_module = module;
                self.details.frag_code = code;
                self.details.frag_mt = mt;
            }

        }

        // recompile
        return true;
    }

    pub fn load_from_details(device: &DeviceBundle, details: StaticShader) -> Result<CompiledShader, String> {

        let vert_mt = StaticShader::file_mt(&details.vert_path);
        let ve = vert_mt.is_some();

        let frag_mt = StaticShader::file_mt(&details.frag_path);
        let fe = frag_mt.is_some();

        if !ve || !fe {
            let mut msg = "Error: The follwing files does not exist!".to_string();
            if !ve {
                msg.push_str(&format!("\n\t {:?}", details.vert_path));
            }

            if !fe {
                msg.push_str(&format!("\n\t {:?}", details.frag_path));
            }

            return Err(msg);
        }

        let vert_mt = match vert_mt {
            Some(mt) => mt,
            None => { return Err("ERROR: LOAD_FROM_DETAILS: UNEXPECTED".to_string()); }
        };

        let frag_mt = match frag_mt {
            Some(mt) => mt,
            None => { return Err("ERROR: LOAD_FROM_DETAILS: UNEXPECTED".to_string()); }
        };


        let vert_result = StaticShader::reload_and_compile_spv(device, &details.vert_spv_path);
        let frag_result = StaticShader::reload_and_compile_spv(device, &details.frag_spv_path);

        if !vert_result.is_some() || !frag_result.is_some() {
            // cleanup and exit

            if let Some((_code, module)) = vert_result {
                unsafe { device.logical.destroy_shader_module(module, None); }
            }

            if let Some((_code, module)) = frag_result {
                unsafe { device.logical.destroy_shader_module(module, None); }
            }

            return Err("Could not create shader modules".to_string());
        }

        match (vert_result, frag_result) {
            (None, None) => {
                Err("Could not create either shader modules".to_string())
            }

            (None, Some((_code, module))) => {
                unsafe { device.logical.destroy_shader_module(module, None); }
                Err("Could not create vertex shader module".to_string())
            }

            (Some((_code, module)), None) => {
                unsafe { device.logical.destroy_shader_module(module, None); }
                Err("Could not create fragment shader module".to_string())
            }

            (Some((vert_code, vert_module)), Some((frag_code, frag_module))) => {
                let mut details = details;
                details.vert_mt = vert_mt;
                details.vert_code = vert_code;

                details.frag_mt = frag_mt;
                details.frag_code = frag_code;

                Ok( Self {
                    details,
                    vert_module,
                    frag_module
                })
            }
        }
    }

}

#[shaders_registry]
pub struct ShaderRegistry {

}

impl ShaderRegistry {

    pub fn describe_registed_shaders() {

        // created by the shader_registry proc_macro_attribute of the struct
        println!("Shader Registry - {} Shaders Registered:", ShaderRegistry::SHADER_DETAILS.len());
        for (name, id, _) in ShaderRegistry::SHADER_DETAILS {
            println!("\t Shader {} ({})", name, id);
        }

    }

}
