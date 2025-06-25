use ash::vk;

use crate::DeviceBundle;


#[allow(unused)]
pub trait Drawable {
    fn dirty(&self) -> bool;
    fn update(&self, device: &DeviceBundle);
    fn record_update(&self, device: &DeviceBundle, command_buffer: &vk::CommandBuffer);
}


pub struct DescSetBinding {
    pub binding: u32,
    pub descriptor_type: vk::DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: vk::ShaderStageFlags,    
}


pub struct PipelineDescriptor {
    pub ubo_layout_bindings: Vec<DescSetBinding>,
    pub vertex_bindings: Vec<vk::VertexInputBindingDescription>,
    pub vertex_attributes: Vec<vk::VertexInputAttributeDescription>
}
