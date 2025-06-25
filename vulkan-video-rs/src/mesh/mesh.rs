#[allow(dead_code)]
pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub dirty_vertices: bool,

    pub colour: Vec<[f32; 3]>,
    pub dirty_colour: bool,

    pub indices: Vec<u16>,
    pub dirty_indices: bool,

    pub original_vertices: Option<Vec<[f32; 3]>>
}


impl Mesh {

    #[allow(dead_code)]
    pub fn transform(&mut self, _rotation: [f32; 3], _translation: [f32; 3]) {
        if self.original_vertices.is_none() {
            self.original_vertices = Some(self.vertices.clone());
        }

        let _orig: &Vec<[f32; 3]> = self.original_vertices.as_ref().unwrap();

        for _i in 0..self.vertices.len() {
            
        }
    }


}
