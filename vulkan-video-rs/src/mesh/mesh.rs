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

    pub fn transform(&mut self, rotation: [f32; 3], translation: [f32; 3]) {
        if self.original_vertices.is_none() {
            self.original_vertices = Some(self.vertices.clone());
        }

        let orig: &Vec<[f32; 3]> = self.original_vertices.as_ref().unwrap();

        for i in 0..self.vertices.len() {
            
        }
    }


}
