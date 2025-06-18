use ash::vk;
use ash::khr;

use crate::mesh::Rect;
use crate::shader::Shader;

pub struct SurfaceBundle {
    pub surface: vk::SurfaceKHR,
    pub loader: khr::surface::Instance
}

pub struct DeviceBundle {
    pub logical: ash::Device,
    pub physical: vk::PhysicalDevice,
    pub queue_family_index: u32,
    pub present_queue: vk::Queue,
    pub mem_properties: vk::PhysicalDeviceMemoryProperties
}

pub struct SwapchainBundle {
    pub swapchain: vk::SwapchainKHR,
    pub loader: khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

pub struct GraphicsPipelineBundle {
    pub shader: Box<dyn Shader>,
    pub graphics: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

pub struct SyncObjectsBundle {
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub spare_fences: Vec<vk::Fence>,
}

pub struct MeshBundle {
    pub mesh: Rect,
    pub vbo: BufferBundle,
    pub staging: BufferBundle,
    pub staging_ind: BufferBundle,
    pub ind: BufferBundle,
}

pub struct BufferBundle {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory
}

pub struct CommandBundle {
    pub pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>
}
