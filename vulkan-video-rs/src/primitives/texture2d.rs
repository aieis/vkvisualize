pub enum PixelFormat {
    RGBA,
    Z16
}

pub struct Texture2d {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,

    pub dirty: bool
}

impl Texture2d {
    pub fn new(data: Vec<u8>, width: u32, height: u32, format: PixelFormat) -> Self {
        Self {
            data,
            width,
            height,
            format,
            dirty: true
        }
    }

    pub fn update_data(&mut self, data: Vec<u8>) {
        self.data = data;
        self.dirty = true;
    }
}
