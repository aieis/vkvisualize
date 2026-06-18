pub struct Cube {
    pub vertices: Vec<[f32; 3]>,
    pub dirty_vertices: bool,

    pub colour: Vec<[f32; 3]>,
    pub dirty_colour: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,
}

impl Cube
{
    pub fn new(x: f32, y: f32, z: f32, length: f32, col: [f32; 3]) -> Self
    {

        let l   = length;
        let l_2 = l / 2.0;
        let x0  = x - l_2;
        let y0  = y - l_2;
        let z0  = z - l_2;

        let vertices = vec![
            // back
            [x0, y0, z0], [x0+l, y0, z0], [x0+l, y0+l, z0], [x0,  y0+l, z0],

            // front
            [x0, y0, z0+l], [x0+l, y0, z0+l], [x0+l, y0+l, z0+l], [x0,  y0+l, z0+l],
        ];

        let colour = vec![
            col, col, col, col,
            col, col, col, col
        ];

        let indices = vec![
            // front
            0, 1, 2, 0, 2, 3,

            // back
            0+4, 1+4, 2+4, 0+4, 2+4, 3+4
        ];

        Self {
            vertices,
            colour,
            indices,
            dirty_vertices: true,
            dirty_colour: true,
            dirty_indices: true,
        }
    }

    pub fn transform(&mut self, rotation: f32, translation: [f32; 2]) {
        let s = rotation.sin();
        let c = rotation.cos();

        for i in 0..self.vertices.len() {
            let x = self.vertices[i][0];
            let y = self.vertices[i][1];
            let z = self.vertices[i][2];


            let xp = x * c - y * s;
            let yp = x * s + y * c;
            self.vertices[i] = [xp + translation[0], yp + translation[1], z];
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
