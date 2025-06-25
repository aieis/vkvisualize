use anyhow::Result;

use ash::vk;

use crate::{DeviceBundle, ImageBundle, TextureBundle};
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


pub fn create_texture_image(device: &DeviceBundle, command_pool: vk::CommandPool, submit_queue: vk::Queue, image_width: u32, image_height: u32, image_size: u64)  -> TextureBundle {
    let required_memory_properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let staging = create_buffer(device, image_size, vk::BufferUsageFlags::TRANSFER_SRC, required_memory_properties).unwrap();

    let format = vk::Format::R8G8B8A8_UINT;
    let resource = create_image(device, image_width, image_height, format,
                             vk::ImageTiling::OPTIMAL,
                             vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
                             vk::MemoryPropertyFlags::DEVICE_LOCAL).unwrap();

    transition_image_layout(device, command_pool, submit_queue, &resource, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
    copy_buffer_to_image(device, command_pool, submit_queue, staging.buffer, resource.image, image_width, image_height);
    transition_image_layout(device, command_pool, submit_queue, &resource, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);

    let sampler = create_sampler(device).unwrap();
    let image_view = create_image_view(device, &resource, vk::ImageAspectFlags::COLOR, 1).unwrap();

    TextureBundle {
        resource,
        staging,
        sampler,
        image_view
    }
}

pub fn transition_image_layout(
    device: &DeviceBundle,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    image: &ImageBundle,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) {
    let command_buffer = begin_single_time_command(device, command_pool);

    let src_access_mask;
    let dst_access_mask;
    let source_stage;
    let destination_stage;

    if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL {
        src_access_mask = vk::AccessFlags::empty();
        dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        source_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = vk::PipelineStageFlags::TRANSFER;
    } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL {
        src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
        dst_access_mask = vk::AccessFlags::SHADER_READ;
        source_stage = vk::PipelineStageFlags::TRANSFER;
        destination_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
    } else if old_layout == vk::ImageLayout::UNDEFINED && new_layout == vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL {
        src_access_mask = vk::AccessFlags::empty();
        dst_access_mask = vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
        source_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
        destination_stage = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
    } else {
        panic!("Unsupported layout transition!")
    }

    let sub_res = vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::COLOR, base_mip_level: 0, level_count: 1,
                                              base_array_layer: 0, layer_count: 1 };

    let image_barriers = [
        vk::ImageMemoryBarrier::default()
            .src_access_mask(src_access_mask)
            .dst_access_mask(dst_access_mask)
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(image.image)
            .subresource_range(sub_res)
    ];

    unsafe {
        device.logical.cmd_pipeline_barrier(command_buffer, source_stage, destination_stage, vk::DependencyFlags::empty()
                                            , &[], &[], &image_barriers);
    }

    end_single_time_command(device, command_pool, submit_queue, command_buffer);
}


pub fn begin_single_time_command(device: &DeviceBundle, command_pool: vk::CommandPool) -> vk::CommandBuffer {
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_buffer_count(1)
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY);

    let command_buffer = unsafe { device.logical.allocate_command_buffers(&command_buffer_allocate_info).unwrap() }[0];

    let command_buffer_begin_info = vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    unsafe { device.logical.begin_command_buffer(command_buffer, &command_buffer_begin_info).unwrap() }

    command_buffer
}


pub fn end_single_time_command(device: &DeviceBundle, command_pool: vk::CommandPool, submit_queue: vk::Queue, command_buffer: vk::CommandBuffer) {
    unsafe { device.logical.end_command_buffer(command_buffer).unwrap() }

    let buffers_to_submit = [command_buffer];

    let sumbit_infos = [ vk::SubmitInfo::default().command_buffers(&buffers_to_submit)];

    unsafe {
        device.logical.queue_submit(submit_queue, &sumbit_infos, vk::Fence::null()).unwrap();
        device.logical.queue_wait_idle(submit_queue).unwrap();
        device.logical.free_command_buffers(command_pool, &buffers_to_submit);
    }
}


pub fn copy_buffer_to_image(
    device: &DeviceBundle,
    command_pool: vk::CommandPool,
    submit_queue: vk::Queue,
    buffer: vk::Buffer,
    image: vk::Image,
    width: u32,
    height: u32,
) {
    let command_buffer = begin_single_time_command(device, command_pool);

    let sub_res = vk::ImageSubresourceLayers {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
    };

    let buffer_image_regions = [
        vk::BufferImageCopy::default()
            .image_subresource(sub_res)
            .image_extent(vk::Extent3D { width, height, depth: 1})
            .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
    ];            

    unsafe {
        device.logical.cmd_copy_buffer_to_image(
            command_buffer,
            buffer,
            image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &buffer_image_regions,
        );
    }

    end_single_time_command(device, command_pool, submit_queue, command_buffer);
}







pub fn format(format: &PixelFormat) -> vk::Format {
    match format {
        PixelFormat::RGBA => vk::Format::R8G8B8A8_UINT,
        PixelFormat::Z16 => vk::Format::D16_UNORM
    }
}
