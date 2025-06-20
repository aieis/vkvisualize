use anyhow::{Result, anyhow};

use ash::vk;

use crate::{BufferBundle, DeviceBundle, ImageBundle};


pub fn create_buffer(device: &DeviceBundle, size: u64, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Result<BufferBundle>{

    let buffer_create_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let buffer = unsafe { device.logical.create_buffer(&buffer_create_info, None)? };
    let mem_requirements = unsafe { device.logical.get_buffer_memory_requirements(buffer) };
    let memory_type = find_memory_type(mem_requirements.memory_type_bits, properties, device.mem_properties)?;

    let allocate_info = vk::MemoryAllocateInfo::default()
        .allocation_size(mem_requirements.size)
        .memory_type_index(memory_type);

    let memory = unsafe { device.logical.allocate_memory(&allocate_info, None)? };

    unsafe { device.logical.bind_buffer_memory(buffer, memory, 0)?; }


    Ok( BufferBundle { buffer, memory } )
}

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

    Ok ( ImageBundle {image, memory} )
}


fn find_memory_type(
    type_filter: u32,
    required_properties: vk::MemoryPropertyFlags,
    mem_properties: vk::PhysicalDeviceMemoryProperties,
) -> Result<u32> {
    for (i, memory_type) in mem_properties.memory_types.iter().enumerate() {
        //if (type_filter & (1 << i)) > 0 && (memory_type.property_flags & required_properties) == required_properties {
        //    return i as u32
        // }

        // same implementation
        if (type_filter & (1 << i)) > 0
            && memory_type.property_flags.contains(required_properties)
        {
            return Ok(i as u32);
        }
    }

    Err(anyhow!("Failed to find memory type."))
}
