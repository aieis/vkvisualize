use anyhow::Result;

use ash::vk;

use crate::{DeviceBundle, ImageBundle};
use crate::primitives::texture2d::PixelFormat;

use super::buffer::create_buffer;
use super::common::find_memory_type;

pub fn create_image(device: &DeviceBundle, width: u32, height: u32, format: vk::Format, tiling: vk::ImageTiling, usage: vk::ImageUsageFlags, properties: vk::MemoryPropertyFlags) -> Result<ImageBundle> {
    let image_ci = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::TYPE_2D)
        .extent(vk::Extent3D {width, height, depth: 1})
        .mip_levels(1)
        .array_layers(1)
        .format(format)
        .tiling(tiling)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(usage)
        .samples(vk::SampleCountFlags::TYPE_1)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let image = unsafe { device.logical.create_image(&image_ci, None)? };
    let mem_requirements = unsafe { device.logical.get_image_memory_requirements(image) };
    let memory_type = find_memory_type(mem_requirements.memory_type_bits, properties, device.mem_properties)?;

    let allocate_info = vk::MemoryAllocateInfo::default()
        .allocation_size(mem_requirements.size)
        .memory_type_index(memory_type);

    let memory = unsafe { device.logical.allocate_memory(&allocate_info, None)? };

    unsafe { device.logical.bind_image_memory(image, memory, 0)?; }

    Ok ( ImageBundle {image, memory, format} )
}

pub fn create_image_view(device: &DeviceBundle, image: &ImageBundle, aspect_flags: vk::ImageAspectFlags, mip_levels: u32) -> Result<vk::ImageView> {
    let imageview_create_info = vk::ImageViewCreateInfo::default()
        .flags(vk::ImageViewCreateFlags::empty())
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(image.format)
        .components(vk::ComponentMapping::default())
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: aspect_flags,
            base_mip_level: 0,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1,
        })
        .image(image.image);

    let image_view = unsafe { device.logical.create_image_view(&imageview_create_info, None)? };
    Ok(image_view)
}

pub fn create_sampler(device: &DeviceBundle) -> Result<vk::Sampler> {
    let sampler_create_info = vk::SamplerCreateInfo::default()
        .flags(vk::SamplerCreateFlags::empty())
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .max_anisotropy(16.0)
        .compare_enable(false)
        .compare_op(vk::CompareOp::ALWAYS)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .min_lod(0.0)
        .max_lod(0.0)
        .mip_lod_bias(0.0)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .anisotropy_enable(true)
        .unnormalized_coordinates(false);

    let sampler = unsafe { device.logical.create_sampler(&sampler_create_info, None)?};
    Ok(sampler)
}


pub fn create_texture_image(device: &DeviceBundle, command_pool: vk::CommandPool, submit_queue: vk::Queue, image_width: u32, image_height: u32, image_size: u64)  {
    let required_memory_properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let staging = create_buffer(device, image_size, vk::BufferUsageFlags::TRANSFER_SRC, required_memory_properties);

    let image = create_image(device, image_width, image_height, vk::Format::R8G8B8A8_UINT,
                             vk::ImageTiling::OPTIMAL,
                             vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                             vk::MemoryPropertyFlags::DEVICE_LOCAL);

    // transition_image_layout(
    //     device,
    //     command_pool,
    //     submit_queue,
    //     texture_image,
    //     vk::Format::R8G8B8A8_SRGB,
    //     vk::ImageLayout::UNDEFINED,
    //     vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    //     1,
    // );

    // copy_buffer_to_image(
    //     device,
    //     command_pool,
    //     submit_queue,
    //     staging_buffer,
    //     texture_image,
    //     image_width,
    //     image_height,
    // );

    // transition_image_layout(
    //     device,
    //     command_pool,
    //     submit_queue,
    //     texture_image,
    //     vk::Format::R8G8B8A8_UNORM,
    //     vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    //     vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    //     1,
    // );

    // unsafe {
    //     device.destroy_buffer(staging_buffer, None);
    //     device.free_memory(staging_buffer_memory, None);
    // }

    // (texture_image, texture_image_memory)
}



pub fn format(format: &PixelFormat) -> vk::Format {
    match format {
        PixelFormat::RGBA => vk::Format::R8G8B8A8_UINT,
        PixelFormat::Z16 => vk::Format::D16_UNORM
    }
}
