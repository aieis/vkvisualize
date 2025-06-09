use anyhow::{Result, anyhow};

use ash::vk;

use crate::DeviceBundle;


pub fn create_buffer(device: &DeviceBundle, size: u64) -> Result<(vk::Buffer, vk::DeviceMemory)>{

    let vertex_buffer_create_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let vertex_buffer = unsafe { device.logical.create_buffer(&vertex_buffer_create_info, None)? };
    let mem_requirements = unsafe { device.logical.get_buffer_memory_requirements(vertex_buffer) };
    let required_memory_flags: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let memory_type = find_memory_type(mem_requirements.memory_type_bits, required_memory_flags, device.mem_properties)?;

    let allocate_info = vk::MemoryAllocateInfo::default()
        .allocation_size(mem_requirements.size)
        .memory_type_index(memory_type);

    let vertex_buffer_memory = unsafe { device.logical.allocate_memory(&allocate_info, None)? };


    Ok((vertex_buffer, vertex_buffer_memory))
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
