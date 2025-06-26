use ash::vk;

use crate::mesh::Rect;
use crate::utils::image::{copy_buffer_to_image, transition_image_layout, ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal, ImageLayout_Undefined};
use crate::vk_base::VkBase;
use crate::vk_bundles::BufferBundle;
use crate::{utils, vk_base, DeviceBundle, GraphicsPipelineBundle, ImageBundle, TextureBundle};
use crate::primitives::texture2d::Texture2d;

use super::drawable_common::{DescSetBinding, PipelineDescriptor};


pub struct DrawableTexture {
    pub rect: Rect,
    pub texture_data: Texture2d,
    pub texture: TextureBundle,
    pub vbo: BufferBundle,
    pub desc_set: Vec<vk::DescriptorSet>,
}

impl DrawableTexture {

    pub fn new(
        device: &DeviceBundle, descriptor_pool: vk::DescriptorPool, command_buffer: vk::CommandBuffer,
        desc_layout: vk::DescriptorSetLayout, swapchain_image_size: usize,
        rect: Rect, texture_data: Texture2d
    ) -> Self {

        let texture = utils::image::create_texture_image(device, texture_data.width, texture_data.height, texture_data.size, utils::image::format(&texture_data.format));

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;
        let usage = vk::BufferUsageFlags::VERTEX_BUFFER;
        let vbo = utils::buffer::create_buffer(device, rect.size_vrt() as u64, usage, required_memory_flags).expect("Failed to create vertex buffer.");

        let desc_set = Self::create_descriptor_sets(device, descriptor_pool, desc_layout, &texture, swapchain_image_size);
        transition_image_layout::<ImageLayout_Undefined, ImageLayout_ShaderReadOnlyOptimal>(device, command_buffer, &texture.resource);
        DrawableTexture { rect, texture_data, texture, vbo, desc_set }
    }

    pub fn dirty(&self) -> bool {
        return self.rect.dirty_colour || self.rect.dirty_indices || self.rect.dirty_vertices || self.texture_data.dirty;
    }

    pub fn update(device: &DeviceBundle, command_buffer: vk::CommandBuffer, entities: &mut Vec<Self>) -> bool {
        let mut recorded = false;

        for entity in entities.iter_mut() {
            if !entity.dirty() {
                continue;
            }

            recorded = true;

            let size_vrt = entity.rect.size_vrt() as u64;
            let texture_size = entity.texture_data.size;

            unsafe {
                if entity.rect.dirty_vertices {
                    let data_ptr = device.logical.map_memory(entity.vbo.memory, 0, size_vrt, vk::MemoryMapFlags::empty()).unwrap() as *mut [f32; 2];
                    data_ptr.copy_from_nonoverlapping(entity.rect.vertices.as_ptr(), entity.rect.vertices.len());
                    device.logical.unmap_memory(entity.vbo.memory);
                }

                if entity.texture_data.dirty {
                    let data_ptr = device.logical.map_memory(entity.texture.staging.memory, 0, texture_size, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
                    data_ptr.copy_from_nonoverlapping(entity.texture_data.data.as_ptr(), texture_size as usize);
                    device.logical.unmap_memory(entity.texture.staging.memory);

                    transition_image_layout::<ImageLayout_ShaderReadOnlyOptimal, ImageLayout_TransferDstOptimal>(device, command_buffer, &entity.texture.resource);
                    copy_buffer_to_image(device, command_buffer, entity.texture.staging.buffer, entity.texture.resource.image, entity.texture_data.width, entity.texture_data.height);
                    transition_image_layout::<ImageLayout_TransferDstOptimal, ImageLayout_ShaderReadOnlyOptimal>(device, command_buffer, &entity.texture.resource);
                }
            }

            entity.rect.dirty_colour = false;
            entity.rect.dirty_vertices = false;
            entity.rect.dirty_indices = false;
        }

        return recorded;
    }

    pub fn draw(device: &DeviceBundle, command_buffer: vk::CommandBuffer, graphics_pipeline: &GraphicsPipelineBundle, entities: &[Self])  {
        unsafe {
            device.logical.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);
            for i in 0..entities.len() {
                device.logical.cmd_bind_vertex_buffers(command_buffer, 0, &[entities[i].vbo.buffer], &[0, 0]);
                device.logical.cmd_draw_indexed(command_buffer, entities[i].rect.indices.len() as u32, 1, 0, 0, 0);
            }
        }
    }

    fn create_descriptor_sets(
        device: &DeviceBundle,
        descriptor_pool: vk::DescriptorPool,
        descriptor_set_layout: vk::DescriptorSetLayout,
        texture: &TextureBundle,
        swapchain_images_size: usize,
    ) -> Vec<vk::DescriptorSet> {
        let mut layouts: Vec<vk::DescriptorSetLayout> = vec![];
        for _ in 0..swapchain_images_size {
            layouts.push(descriptor_set_layout);
        }

        let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.logical.allocate_descriptor_sets(&descriptor_set_allocate_info).unwrap() };

        for &descriptor_set in descriptor_sets.iter() {
            let descriptor_image_infos = [vk::DescriptorImageInfo {
                sampler: texture.sampler,
                image_view: texture.image_view,
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            }];

            let descriptor_write_sets = [
                vk::WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .image_info(&descriptor_image_infos)
            ];

            unsafe {
                device.logical.update_descriptor_sets(&descriptor_write_sets, &[]);
            }
        }

        descriptor_sets

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
