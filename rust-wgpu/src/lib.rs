use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2]
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS
        }
    }
}
pub const VERTICES: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 0.0], tex_coords: [1.0, 1.0], },
    Vertex { position: [ 1.0, -1.0, 0.0], tex_coords: [0.0, 1.0], },
    Vertex { position: [ 1.0,  1.0, 0.0], tex_coords: [0.0, 0.0], },
    Vertex { position: [-1.0,  1.0, 0.0], tex_coords: [1.0, 0.0], }
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    3, 0, 2
];
