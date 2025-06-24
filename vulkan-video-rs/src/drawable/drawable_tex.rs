use ash::vk;

use crate::mesh::Rect;
use crate::vk_bundles::BufferBundle;
use crate::{vk_utils, DeviceBundle, GraphicsPipelineBundle, ImageBundle};
use crate::primitives::texture2d::Texture2d;


pub struct DrawableTexture {
    pub rect: Rect,
    pub texture: Texture2d,
    pub image: ImageBundle,
    pub vbo: BufferBundle,
}

impl DrawableTexture {

    pub fn new(device: &DeviceBundle, rect: Rect, texture: Texture2d) -> Self {

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        let usage = vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
        let image = vk_utils::create_image(device, texture.width, texture.height, vk_utils::format(&texture.format), vk::ImageTiling::OPTIMAL, usage, required_memory_flags).unwrap();

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = vk_utils::create_buffer(device, rect.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        DrawableTexture { rect, texture, image, vbo }
    }

    pub fn dirty(&self) -> bool {
        return self.rect.dirty_colour || self.rect.dirty_indices || self.rect.dirty_vertices || self.texture.dirty;
    }

    pub fn update(device: &DeviceBundle, _command_buffer: &vk::CommandBuffer, mesh_bundles: &mut Vec<Self>) -> bool {

        let mut recorded = false;

        for mesh_bundle in mesh_bundles.iter_mut() {
            if !mesh_bundle.dirty() {
                continue;
            }

            recorded = true;

            let size_vrt = mesh_bundle.rect.size_vrt() as u64;

            unsafe {
                if mesh_bundle.rect.dirty_vertices {
                    let data_ptr = device.logical.map_memory(mesh_bundle.vbo.memory, 0, size_vrt, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(mesh_bundle.rect.vertices.as_ptr(), mesh_bundle.rect.vertices.len());
                    device.logical.unmap_memory(mesh_bundle.vbo.memory);
                }
            }

            mesh_bundle.rect.dirty_colour = false;
            mesh_bundle.rect.dirty_vertices = false;
            mesh_bundle.rect.dirty_indices = false;
        }

        return recorded;
    }

    pub fn draw(device: &DeviceBundle, command_buffer: &vk::CommandBuffer, graphics_pipeline: &GraphicsPipelineBundle, mesh_bundles: &[Self])  {
        let command_buffer = *command_buffer;
        unsafe {
            device.logical.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);
            for i in 0..mesh_bundles.len() {
                device.logical.cmd_bind_vertex_buffers(command_buffer, 0, &[mesh_bundles[i].vbo.buffer], &[0, 0]);
                device.logical.cmd_draw_indexed(command_buffer, mesh_bundles[i].rect.indices.len() as u32, 1, 0, 0, 0);
            }
        }
    }
}
