use ash::vk;
use ash::khr;

pub struct SurfaceBundle {
    pub surface: vk::SurfaceKHR,
    pub loader: khr::surface::Instance
}

pub struct DeviceBundle {
    pub logical: ash::Device,
    pub physical: vk::PhysicalDevice,
    pub queue_family_index: u32,
}

impl DeviceBundle {
    pub fn queue(&self, queue_index: u32) -> vk::Queue {
        unsafe { self.logical.get_device_queue(self.queue_family_index, queue_index) }
    }    
}

pub struct SwapchainBundle {
    pub swapchain: vk::SwapchainKHR,
    pub loader: khr::swapchain::Device,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

pub struct GraphicsPipelineBundle {
    pub graphics: vk::Pipeline,
    pub layout: vk::PipelineLayout
}

pub struct SyncObjectsBundle {
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub inflight_fences: Vec<vk::Fence>,
}
