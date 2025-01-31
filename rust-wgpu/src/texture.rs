
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler
}

impl Texture {
    pub fn from_bytes(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat, label: &str,
                      bytes: &[u8], width: u32, height: u32, stride: u32) -> Self {
        let size = wgpu::Extent3d {width, height, depth_or_array_layers: 1};
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some(label),
            view_formats: &[]
        });

        let write_data = wgpu::ImageCopyTexture {
            texture: &texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All
        };
        let write_layout = wgpu::ImageDataLayout {
            offset: 0, bytes_per_row: Some(stride), rows_per_image: Some(height)
        };

        queue.write_texture(write_data, &bytes, write_layout, size);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge, address_mode_v: wgpu::AddressMode::ClampToEdge, address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear, min_filter: wgpu::FilterMode::Nearest, mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, bytes: &[u8]) {
        let write_data = wgpu::ImageCopyTexture {
            texture: &self.texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All
        };

        let write_layout = wgpu::ImageDataLayout {
            offset: 0, bytes_per_row: Some(self.texture.width() * 2), rows_per_image: Some(self.texture.height())
        };
        
        queue.write_texture(write_data, &bytes, write_layout, self.texture.size());
    }
}
