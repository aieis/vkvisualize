use ash::vk;

use crate::mesh::Rect;
use crate::vk_bundles::BufferBundle;
use crate::{utils, DeviceBundle, GraphicsPipelineBundle, ImageBundle};
use crate::primitives::texture2d::Texture2d;

use super::drawable_common::{DescSetBinding, PipelineDescriptor};


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
        let image = utils::image::create_image(device, texture.width, texture.height, utils::image::format(&texture.format), vk::ImageTiling::OPTIMAL, usage, required_memory_flags).unwrap();

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = utils::buffer::create_buffer(device, rect.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

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

    fn create_descriptor_sets(
        device: &DeviceBundle,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        texture_image_view: vk::ImageView,
        texture_sampler: vk::Sampler,
        swapchain_images_size: usize,
    ) -> Vec<vk::DescriptorSet> {
        vec![]
    }

    pub fn pipeline_descriptor() -> PipelineDescriptor {
        let ubo_layout_bindings = vec![
            DescSetBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::FRAGMENT,
            }
        ];

        let vertex_bindings = vec![
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),

            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(std::mem::size_of::<[f32; 2]>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
        ];

        let vertex_attributes = vec![
            vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT),
            vk::VertexInputAttributeDescription::default()
                .binding(1)
                .location(1)
                .format(vk::Format::R32G32_SFLOAT)
        ];

        PipelineDescriptor {
            ubo_layout_bindings,
            vertex_bindings,
            vertex_attributes,
        }
    }
}
