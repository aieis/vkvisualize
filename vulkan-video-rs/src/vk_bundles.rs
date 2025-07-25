use ash::vk;
use ash::khr;

use crate::drawable::drawable_common::PipelineDescriptor;
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
    pub ubo: Option<Vec<vk::DescriptorSetLayout>>,
    pub pipeline_desc: PipelineDescriptor
}

pub struct SyncObjectsBundle {
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub spare_fences: Vec<vk::Fence>,
}

pub struct BufferBundle {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory
}

pub struct ImageBundle {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    pub format: vk::Format,
}


pub struct CommandBundle {
    pub pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>
}

pub struct TextureBundle {
    pub resource: ImageBundle,
    pub sampler: vk::Sampler,
    pub image_view: vk::ImageView,
    pub aspect_flags: vk::ImageAspectFlags,
    pub staging: BufferBundle,
}
