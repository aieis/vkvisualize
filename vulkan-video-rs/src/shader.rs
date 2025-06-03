use ash::vk;


pub struct Shader {
    _vertex_path: String,
    pub vertex: vk::ShaderModule,

    _fragment_path: String,
    pub fragment: vk::ShaderModule
}


impl Shader {
    pub fn new(device: &ash::Device, vertex_path: &str, fragment_path: &str) -> Self {
	let code = std::fs::read(vertex_path).unwrap();
	let create_info = vk::ShaderModuleCreateInfo {
	    code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
	    ..Default::default()
	};

	let vertex = unsafe { device.create_shader_module(&create_info, None).unwrap() };

	let code = std::fs::read(fragment_path).unwrap();
	let create_info = vk::ShaderModuleCreateInfo {
	    code_size: code.len(),
            p_code: code.as_ptr() as *const u32,
	    ..Default::default()
	};

	let fragment = unsafe { device.create_shader_module(&create_info, None).unwrap() };

	Self {
	    _vertex_path: vertex_path.into(),
	    vertex,

	    _fragment_path: fragment_path.into(),
	    fragment
	}

    }
}
