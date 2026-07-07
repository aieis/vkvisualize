use crate::geometry::vec3::{self, Vec3};

pub struct Mesh {
    pub center: Vec3,

    pub vertices: Vec<Vec3>,
    pub dirty_vertices: bool,

    pub colour: Vec<Vec3>,
    pub dirty_colour: bool,

    pub normals: Vec<Vec3>,
    pub dirty_normals: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,
}


impl Mesh {

    pub fn rotate_z(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x * c - y * s,
                y: cn.y + x * s + y * c,
                z: cn.z + z
            };
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_y(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x * c - z * s,
                y: cn.y + y,
                z: cn.z + x * s + z * c,
            };
        }

        self.dirty_vertices = true;
    }

    pub fn rotate_x(&mut self, theta: f32) {

        let (s, c) = theta.sin_cos();

        let cn = &self.center;

        for i in 0..self.vertices.len() {
            let v = &self.vertices[i];
            let [x, y, z] = [v.x - cn.x, v.y - cn.y, v.z - cn.z];

            self.vertices[i] = Vec3 {
                x: cn.x + x,
                y: cn.y + z * s + y * c,
                z: cn.z + z * c - y * s,
            };
        }

        self.dirty_vertices = true;
    }

    pub fn recompute_normals(&mut self) {

        let NUM_VERTS: usize = self.vertices.len();
        let NUM_TRIS : usize = self.indices.len() / 3;

        for t in 0..NUM_TRIS {
            let i0 = self.indices[t * 3 + 0];
            let i1 = self.indices[t * 3 + 1];
            let i2 = self.indices[t * 3 + 2];

            let v0 = self.vertices[i0 as usize];
            let v1 = self.vertices[i1 as usize];
            let v2 = self.vertices[i2 as usize];

            let n  = Vec3::cross(v1 - v0, v2 - v0);

            self.normals[i0 as usize] += n;
            self.normals[i1 as usize] += n;
            self.normals[i2 as usize] += n;
        }

        for i in 0..NUM_VERTS {
            self.normals[i] = Vec3::norm(&self.normals[i]);
        }

    }


    pub fn set_colour(&mut self, colour: [f32; 3]) {
        self.colour.fill(Vec3::from_slice(colour));
        self.dirty_colour = true;
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
