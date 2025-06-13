use std::mem::offset_of;

use ash::vk;


#[repr(C)]
#[derive(Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub col: [f32; 3]
}

fn same_color(a: &[f32; 3], b: &[f32; 3], delta: f32) -> bool {
    return (a[0] - b[0] < delta || a[0] - b[0] < delta)
        && (a[1] - b[1] < delta || a[1] - b[1] < delta)
        && (a[2] - b[2] < delta || a[2] - b[2] < delta);
}

fn interpolate(a: &[f32; 3], b: &[f32; 3], c: &[f32; 3], delta: f32) -> [f32; 3] {
    return [(b[0] - a[0]) * delta + c[0],
            (b[1] - a[1]) * delta + c[1],
            (b[2] - a[2]) * delta + c[2]];
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,


    pub original_vertices: Option<Vec<Vertex>>
}

impl Mesh {

    pub fn hue_shift(&mut self) {

        if self.original_vertices.is_none() {
            self.original_vertices = Some(self.vertices.clone())
        }

        let orig = self.original_vertices.as_ref().unwrap();
        let delta = 0.00005;
        for i in 0..self.vertices.len() {
            let target_idx = (i + 1) % self.vertices.len();

            let next_col = orig[target_idx].col;
            if same_color(&self.vertices[i].col, &next_col, 0.001) {
                self.vertices[i].col = orig[i].col;
            } else {
                self.vertices[i].col = interpolate(&orig[i].col, &next_col, &self.vertices[i].col, delta);
            }


        }

    }

    pub fn size(&self) -> usize {
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

    pub fn triangle() -> Self {
        let vertices = vec![
            Vertex {
                pos: [-0.5, -0.5],
                col: [1.0, 1.0, 0.0],
            },
            Vertex {
                pos: [0.5, -0.5],
                col: [0.0, 1.0, 1.0],
            },
            Vertex {
                pos: [0.5, 0.5],
                col: [1.0, 0.0, 1.0],
            },

            Vertex {
                pos: [-0.5, 0.5],
                col: [1.0, 0.0, 0.0],
            }
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Self {
            vertices,
            indices,
            original_vertices: None
        }
    }

}
