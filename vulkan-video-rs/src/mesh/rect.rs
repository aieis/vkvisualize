use std::mem::offset_of;

use ash::vk;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub col: [f32; 3]
}

pub struct Rect {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub original_vertices: Option<Vec<Vertex>>
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, col: [f32; 3]) -> Self {
        let vertices = vec![
            Vertex { pos: [x, y], col},
            Vertex { pos: [x+width, y], col},
            Vertex { pos: [x+width, y+height], col},
            Vertex { pos: [x, y+height], col}
        ];


        let indices = vec![0, 1, 2, 0, 2, 3];

        Self {
            vertices,
            indices,
            original_vertices: None
        }
    }

    pub fn size_vrt(&self) -> usize {
        std::mem::size_of_val(&self.vertices[..])
    }

    pub fn size_ind(&self) -> usize {
        std::mem::size_of_val(&self.indices[..])
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
}
