use std::mem::offset_of;
use rand::prelude::*;

use ash::vk;


#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub col: [f32; 3]
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    // pub original_vertices: Option<Vec<Vertex>>
}


impl Mesh {

    pub fn hue_shift(&mut self) {

        // if self.original_vertices.is_none() {
        //     self.original_vertices = Some(self.vertices.clone())
        // }

        let mut rng = rand::rng();

        for vertex in self.vertices.iter_mut() {
            let next_color: [f32; 3] = [
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0),
                rng.random_range(0.0..=1.0)
            ];

            vertex.col[0] = vertex.col[0] + (next_color[0] - vertex.col[0]) * 0.001;
            vertex.col[1] = vertex.col[1] + (next_color[0] - vertex.col[1]) * 0.001;
            vertex.col[2] = vertex.col[2] + (next_color[0] - vertex.col[2]) * 0.001;
        }

    }

    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.vertices[..])
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
                col: [1.0, 1.0, 0.0],
            },
            Vertex {
                pos: [0.5, 0.5],
                col: [0.0, 1.0, 1.0],
            },
            Vertex {
                pos: [-0.5, 0.5],
                col: [1.0, 0.0, 1.0],
            },
        ];

        Self {
            vertices,
            // original_vertices: None
        }
    }

}
