use ash::vk;

#[allow(unused)]
pub trait Drawable {
    fn needs_update(&self) -> bool;
    fn update(&self);
    fn record_update(&self, command_buffer: &vk::CommandBuffer);
}
