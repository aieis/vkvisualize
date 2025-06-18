use ash::vk;

pub struct Rect {
    pub vertices: Vec<[f32; 2]>,
    pub colour: Vec<[f32; 3]>,
    pub indices: Vec<u16>,
    pub original_vertices: Option<Vec<[f32; 2]>>
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, col: [f32; 3]) -> Self {
        let vertices = vec![
            [x, y],
            [x+width, y],
            [x+width, y+height],
            [x, y+height]
        ];

        let colour = vec![
            col,
            col,
            col,
            col
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Self {
            vertices,
            colour,
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

    pub fn size_col(&self) -> usize {
        std::mem::size_of_val(&self.colour[..])
    }

    pub fn get_binding_descriptions() -> [vk::VertexInputBindingDescription; 2] {
        [
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 3]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
        ]
    }
}
