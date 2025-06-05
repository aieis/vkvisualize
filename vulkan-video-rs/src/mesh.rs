use std::mem::offset_of;

use ash::vk;


#[repr(C)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub col: [f32; 3]
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
}


impl Mesh {
    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.vertices)
    }
    pub fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        }]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                binding: 0,
                location: 1,
                format: vk::Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, col) as u32,
            },
        ]
    }

    pub fn triangle() -> Self {
        let vertices = vec![
            Vertex {
                pos: [0.0, -0.5],
                col: [1.0, 0.0, 0.0],
            },
            Vertex {
                pos: [0.5, 0.5],
                col: [0.0, 1.0, 0.0],
            },
            Vertex {
                pos: [-0.5, 0.5],
                col: [0.0, 0.0, 1.0],
            },
        ];

        Self {
            vertices
        }
    }

}
