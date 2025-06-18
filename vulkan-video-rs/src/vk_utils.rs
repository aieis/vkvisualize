use anyhow::{Result, anyhow};

use ash::vk;

use crate::{mesh::Rect, BufferBundle, DeviceBundle, MeshBundle};
use crate::vk_utils;


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


pub fn create_mesh_bundle(device: &DeviceBundle, mesh: Rect) -> MeshBundle {
    let size = mesh.size_vrt() as u64;

    let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let usage = vk::BufferUsageFlags::TRANSFER_SRC;
    let staging = vk_utils::create_buffer(device, size, usage, required_memory_flags).expect("Failed to create vertex buffer.");

    let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
    let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER;
    let vbo = vk_utils::create_buffer(device, size, usage, required_memory_flags).expect("Failed to create vertex buffer.");

    let size = mesh.size_ind() as u64;
    let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    let usage = vk::BufferUsageFlags::TRANSFER_SRC;
    let staging_ind = vk_utils::create_buffer(device, size, usage, required_memory_flags).expect("Failed to create vertex buffer.");

    let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
    let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER;
    let ind = vk_utils::create_buffer(device, size, usage, required_memory_flags).expect("Failed to create vertex buffer.");

    MeshBundle { mesh, vbo, staging, staging_ind, ind}
}


pub fn record_mesh_update(device: &DeviceBundle, command_buffer: &vk::CommandBuffer, mesh_bundles: &[MeshBundle]) {

    let command_buffer_begin_info =  vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    unsafe {
        device.logical.begin_command_buffer(*command_buffer, &command_buffer_begin_info).expect("Failed to begin buffer.");

        for mesh_bundle in mesh_bundles {
            let copy_region = [ vk::BufferCopy::default().size(mesh_bundle.mesh.size_vrt() as u64)];
            device.logical.cmd_copy_buffer(*command_buffer, mesh_bundle.staging.buffer, mesh_bundle.vbo.buffer, &copy_region);

            let copy_region = [ vk::BufferCopy::default().size(mesh_bundle.mesh.size_ind() as u64)];
            device.logical.cmd_copy_buffer(*command_buffer, mesh_bundle.staging_ind.buffer, mesh_bundle.ind.buffer, &copy_region);
        }

        device.logical.end_command_buffer(*command_buffer).expect("Failed to end buffer.");
    }
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
