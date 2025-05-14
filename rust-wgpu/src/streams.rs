use crate::{k4a, texture::Texture};



pub struct Depth {
    pub feed: Texture,
    pub proj: Texture
}

impl Depth {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, im: k4a::Image, pcl: k4a::Image) -> Self {
        let label_l = format!("{}-feed", label);
        let feed = Texture::from_bytes(
            &device, &queue, wgpu::TextureFormat::Depth16Unorm, &label_l,
            im.get_buffer(), im.width as u32, im.height as u32, im.width as u32 * 2
        );

        let label_l = format!("{}-proj", label);
        let proj = Texture::from_bytes(
            &device, &queue, wgpu::TextureFormat::Depth16Unorm, &label_l,
            pcl.get_buffer(), pcl.width as u32, pcl.height as u32, pcl.width as u32 * 2
        );

        Self {
            feed,
            proj
        }
    }
}
