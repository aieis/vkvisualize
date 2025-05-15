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
