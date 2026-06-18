pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub dirty_vertices: bool,

    pub colour: Vec<[f32; 3]>,
    pub dirty_colour: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,
}


impl Mesh {

    pub fn size_vrt(&self) -> usize {
        std::mem::size_of_val(&self.vertices[..])
    }

    pub fn size_ind(&self) -> usize {
        std::mem::size_of_val(&self.indices[..])
    }

    pub fn size_col(&self) -> usize {
        std::mem::size_of_val(&self.colour[..])
    }
}
