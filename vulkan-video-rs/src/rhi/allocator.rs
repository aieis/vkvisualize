use ash::vk;

use crate::{utils::buffer, vk_base::VkBase, vk_bundles::{BufferBundle, DeviceBundle}};


enum BufferType {

    /* Properties: HOST_VISIBLE | HOST_COHERENT
     * Usage: TRANSFER_SRC */
    Staging,


    /* Properties: DEVICE_LOCAL
     * Usage: TRANSFER_DST | VERTEX_BUFFER */
    DeviceVertex,

    /* Properties: DEVICE_LOCAL
     * Usage: TRANSFER_DST | Index_BUFFER */
    DeviceIndex

}


struct AllocatorHeap {
    heap   : BufferBundle,
    offset : u32,
    size   : u32
}

struct Allocator {
    heap_staging       : AllocatorHeap,
    heap_device_vertex : AllocatorHeap,
    heap_device_index  : AllocatorHeap,
}


impl Allocator {


    pub fn new(base: VkBase, size: u32) -> Self {

        let offset = 0;

        let required_memory_flags = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC;
        let heap = buffer::create_buffer(&base.device, size as u64, usage, required_memory_flags).expect("Failed to create buffer.");
        let heap_staging = AllocatorHeap { heap, offset, size };


        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER;
        let heap = buffer::create_buffer(&base.device, size as u64, usage, required_memory_flags).expect("Failed to create buffer.");
        let heap_device_vertex = AllocatorHeap { heap, offset, size };

        let required_memory_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let usage = vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER;
        let heap = buffer::create_buffer(&base.device, size as u64, usage, required_memory_flags).expect("Failed to create buffer.");
        let heap_device_index = AllocatorHeap { heap, offset, size };


        Self {
            heap_staging,
            heap_device_vertex,
            heap_device_index
        }
    }

    pub fn alloc(&mut self, buffer_type: BufferType, size: u32) -> Result<BufferBundle, String> {

        let heap = match buffer_type {
            BufferType::Staging => &mut self.heap_staging,
            BufferType::DeviceVertex => &mut self.heap_device_vertex,
            BufferType::DeviceIndex => &mut self.heap_device_index
        };

        if heap.offset + size >= heap.size {
            return Err(format!("Allocator: Requested {} bytes from heap of size {} with {} remaining", size, heap.size, heap.size - heap.offset));
        }

        let bundle = BufferBundle {
            buffer: heap.heap.buffer,
            memory: heap.heap.memory,
            offset: heap.offset,
        };

        heap.offset += size;

        Ok(bundle)
    }


    pub fn release(&mut self, device: &DeviceBundle) {
        unsafe {
            device.logical.destroy_buffer(self.heap_staging.heap.buffer, None);
            device.logical.free_memory(self.heap_staging.heap.memory, None);
            self.heap_staging.offset = 0;
            self.heap_staging.size = 0;

            device.logical.destroy_buffer(self.heap_device_vertex.heap.buffer, None);
            device.logical.free_memory(self.heap_device_vertex.heap.memory, None);
            self.heap_device_vertex.offset = 0;
            self.heap_device_vertex.size = 0;

            device.logical.destroy_buffer(self.heap_device_index.heap.buffer, None);
            device.logical.free_memory(self.heap_device_index.heap.memory, None);
            self.heap_device_index.offset = 0;
            self.heap_device_index.size = 0;

        }
    }
}
