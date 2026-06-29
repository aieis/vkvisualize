pub struct Mesh {
    pub center: [f32; 3],
    pub vertices: Vec<[f32; 3]>,
    pub dirty_vertices: bool,

    pub colour: Vec<[f32; 3]>,
    pub dirty_colour: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,
}


impl Mesh {

    pub fn rotate_z(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let [cx, cy, cz] = self.center;

        for i in 0..self.vertices.len() {
            let [x, y, z] = self.vertices[i];
            let [x, y, z] = [x - cx, y - cy, z - cz];

            self.vertices[i] = [
                cx + x * c - y * s,
                cy + x * s + y * c,
                cz + z
            ];
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_y(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let [cx, cy, cz] = self.center;

        for i in 0..self.vertices.len() {
            let [x, y, z] = self.vertices[i];
            let [x, y, z] = [x - cx, y - cy, z - cz];

            self.vertices[i] = [
                cx + x * c - z * s,
                cy + y,
                cz + x * s + z * c,
            ];
        }

        self.dirty_vertices = true;
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
}
