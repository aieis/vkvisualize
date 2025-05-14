use ash::vk;
use ash::khr;

pub struct DeviceBundle {
    device: ash::Device,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
}

impl DeviceBundle {
    pub fn queue(&self, queue_index: u32) -> vk::Queue {
        unsafe { self.device.get_device_queue(self.queue_family_index, queue_index) }
    }    
}

pub struct SwapchainBundle {
    swapchain_loader: khr::swapchain::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_images: Vec<vk::Image>,
    swapchain_format: vk::Format,
    swapchain_extent: vk::Extent2D,
}
