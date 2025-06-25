use ash::vk;

fn compile_shader_modules(device: &ash::Device, vertex_code: &[u8], fragment_code: &[u8]) -> (vk::ShaderModule, vk::ShaderModule) {
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
    pub fragment_path: String
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
    ($x: literal) => {
        {
            let vertex_code = include_bytes!(concat!("../assets/shaders/", $x, ".vert.spv")).to_vec();
            let fragment_code = include_bytes!(concat!("../assets/shaders/", $x, ".frag.spv")).to_vec();
            ShaderComp { vertex_code, fragment_code  }
        }
    }
}
