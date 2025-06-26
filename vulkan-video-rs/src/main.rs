mod vk_base;
mod vk_bundles;
mod shader;
mod mesh;
mod shader_utils;
mod devices;
mod drawable;
mod primitives;
mod utils;

use drawable::{drawable2d::Drawable2d, drawable_tex::DrawableTexture};
use mesh::Rect;
use primitives::texture2d::{PixelFormat, Texture2d};
use utils::image::{begin_single_time_command, end_single_time_command};
use vk_bundles::*;

use ash::vk;
use simple_logger::SimpleLogger;

use vk_base::VkBase;

use shader::ShaderComp;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

struct App {
    base: VkBase,
    mesh_bundles: Vec<Drawable2d>,
    textures: Vec<DrawableTexture>,
    graphics_pipelines: Vec<GraphicsPipelineBundle>,
    close: bool,
}

impl App {
    fn new(window: Window) -> Self {
        let base = VkBase::new(window, 3);

        let graphics_pipelines = vec![
            base.create_graphics_pipeline(Drawable2d::pipeline_descriptor(), Box::from(make_shader!("triangle"))),
            base.create_graphics_pipeline(DrawableTexture::pipeline_descriptor(), Box::from(make_shader!("texture")))
        ];

        let mesh_bundles = vec![
            Drawable2d::new(&base.device, Rect::new(-0.9, -0.9, 0.5, 0.5, [1.0, 0.0, 0.0])),
            Drawable2d::new(&base.device, Rect::new(0.0, 0.0, 0.5, 0.5, [0.0, 0.0, 1.0])),
            Drawable2d::new(&base.device, Rect::new(-0.25, -0.25, 0.5, 0.5, [0.0, 1.0, 1.0]))
        ];

        let data: [u8; 320 * 320 * 4] = [255; 320 *320 * 4];
        let texture = Texture2d::new(data.to_vec(), 320, 320, PixelFormat::RGBA);

        //TODO: Cleanup descriptor pool

        let command_buffer = begin_single_time_command(&base.device, base.spare_command.pool);
        let ubo = graphics_pipelines[1].ubo.as_ref().unwrap();

        let textures = vec![
            DrawableTexture::new(&base.device, base.descriptor_pool,  command_buffer, ubo[0], base.swapchain.images.len(), Rect::new(0.5, 0.5, 0.2, 0.2, [1.0, 1.0, 1.0]), texture)
        ];

        end_single_time_command(&base.device, base.spare_command.pool, base.device.present_queue, command_buffer);

        Self {
            base,
            mesh_bundles,
            textures,
            graphics_pipelines,
            close: false
        }
    }

    fn update(&mut self) {

        for mesh_bundle in self.mesh_bundles.iter_mut() {
            mesh_bundle.mesh.transform(0.001, [0.0, 0.0]);
        }

        self.base.cleanup_in_flight_buffers();

        if self.base.sync_objects.spare_fences.len() == 0 {
            return;
        }

        let command_buffer = self.base.spare_command.buffers.pop();
        if command_buffer.is_none() {
            return;
        }

        let command_buffers = [command_buffer.unwrap()];

        let command_buffer_begin_info =  vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            let _ = self.base.device.logical.reset_command_buffer(command_buffers[0], vk::CommandBufferResetFlags::empty());
            self.base.device.logical.begin_command_buffer(command_buffers[0], &command_buffer_begin_info).unwrap();
        }

        let res = Drawable2d::update(&self.base.device, &command_buffers[0], &mut self.mesh_bundles);
        DrawableTexture::update(&self.base.device, command_buffers[0], &mut self.textures);

        unsafe { self.base.device.logical.end_command_buffer(command_buffers[0]).unwrap(); }

        if ! res {
            self.base.spare_command.buffers.push(command_buffers[0]);
            return;
        }

        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&command_buffers);

        let fences = [self.base.sync_objects.spare_fences.pop().unwrap()];
        unsafe {
            self.base.device.logical.reset_fences(&fences).expect("Failed to reset fences.");
	    self.base.device.logical.queue_submit(self.base.device.present_queue, &[submit_info], fences[0]).expect("Failure submitting to the queue.");
        }

        self.base.in_flight_buffers.push((command_buffers[0], fences[0]));
    }

    fn render(&mut self) {
        let wait_fences = [self.base.sync_objects.in_flight_fences[self.base.current_frame]];

        let (image_index, _is_sub_optimal) = unsafe {
            self.base.device.logical.wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!");

            let result = self.base.swapchain.loader.acquire_next_image(
                self.base.swapchain.swapchain, std::u64::MAX,
                self.base.sync_objects.image_available_semaphores[self.base.current_frame],
                vk::Fence::null());

            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain_and_pipelines();
                        return;
                    }
                    _ => panic!("Failed to acquire swapchain image!"),
                },
            }
        };

        let wait_semaphores = [self.base.sync_objects.image_available_semaphores[self.base.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.base.sync_objects.render_finished_semaphores[self.base.current_frame]];

        let command_buffers = [self.base.commands[image_index as usize].buffers[0]];

        unsafe {
            self.base.device.logical.reset_command_buffer(command_buffers[0], vk::CommandBufferResetFlags::empty()).expect("Failed to reset command buffer.");
        };

        self.base.begin_renderpass_command_buffer(&command_buffers[0], &self.base.framebuffers[image_index as usize]);
        Drawable2d::draw(&self.base.device, &command_buffers[0], &self.graphics_pipelines[0], &self.mesh_bundles);
        self.base.end_command_buffer(&command_buffers[0]);

        let submit_infos = [
	    vk::SubmitInfo::default()
		.wait_semaphores(&wait_semaphores)
		.wait_dst_stage_mask(&wait_stages)
		.command_buffers(&command_buffers)
		.signal_semaphores(&signal_semaphores)
        ];

        unsafe {
            self.base.device.logical.reset_fences(&wait_fences).expect("Failed to reset Fence!");
	    self.base.device.logical.queue_submit(self.base.device.present_queue, &submit_infos, self.base.sync_objects.in_flight_fences[self.base.current_frame],)
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [self.base.swapchain.swapchain];

	let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
	    .wait_semaphores(&signal_semaphores)
	    .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result =  unsafe { self.base.swapchain.loader.queue_present(self.base.device.present_queue, &present_info) };

        let is_resized = match result {
            Ok(_) => self.base.is_framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present."),
            },
        };

        self.base.current_frame = (self.base.current_frame + 1) % self.base.max_in_flight;

        if is_resized {
            self.base.is_framebuffer_resized = false;
            self.base.recreate_swapchain();
            return self.render();
        }
    }

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.close = true;
            }
            WindowEvent::RedrawRequested => {
                self.base.window.pre_present_notify();
                self.update();
                self.render();
            }

            WindowEvent::Resized(_) => {
                self.base.window.request_redraw();
            }

            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.handle_key(event);
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        match event.physical_key {
            winit::keyboard::PhysicalKey::Code(KeyCode::KeyQ) => {
                self.close = true;
            }
            winit::keyboard::PhysicalKey::Code(KeyCode::Escape) => {
                self.close = true;
            }
            _ => {}
        }
    }

    fn recreate_swapchain_and_pipelines(&mut self) {
        // TODO: Pushing and popping will be bad when there are more graphics

        self.base.recreate_swapchain();

        let count = self.graphics_pipelines.len();

        let mut new_pipes = Vec::new();

        for _ in 0..count {
            let graphics_pipeline = self.graphics_pipelines.remove(0);
            unsafe {
                self.base.device.logical.destroy_pipeline(graphics_pipeline.graphics, None);
                self.base.device.logical.destroy_pipeline_layout(graphics_pipeline.layout, None);
            }

            let graphics_pipeline = self.base.recreate_graphics_pipeline(graphics_pipeline);
            new_pipes.push(graphics_pipeline);
        }

        self.graphics_pipelines = new_pipes;
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {

            let _ = self.base.device.logical.device_wait_idle();


            for mesh in self.mesh_bundles.iter() {
                self.base.device.logical.destroy_buffer(mesh.vbo.buffer, None);
                self.base.device.logical.free_memory(mesh.vbo.memory, None);
                self.base.device.logical.destroy_buffer(mesh.staging.buffer, None);
                self.base.device.logical.free_memory(mesh.staging.memory, None);
                self.base.device.logical.destroy_buffer(mesh.col.buffer, None);
                self.base.device.logical.free_memory(mesh.col.memory, None);
                self.base.device.logical.destroy_buffer(mesh.ind.buffer, None);
                self.base.device.logical.free_memory(mesh.ind.memory, None);
            }

            for texture in self.textures.iter() {
                self.base.device.logical.destroy_buffer(texture.vbo.buffer, None);
                self.base.device.logical.free_memory(texture.vbo.memory, None);
                self.base.device.logical.destroy_buffer(texture.texture.staging.buffer, None);
                self.base.device.logical.free_memory(texture.texture.staging.memory, None);
                self.base.device.logical.destroy_image(texture.texture.resource.image, None);
                self.base.device.logical.free_memory(texture.texture.resource.memory, None);
                self.base.device.logical.destroy_image_view(texture.texture.image_view, None);
                self.base.device.logical.destroy_sampler(texture.texture.sampler, None);
            }

            for i in 0..self.graphics_pipelines.len() {
                if let Some(ubo) = self.graphics_pipelines[i].ubo.as_ref() {
                    for ubo_elem in ubo {
                        self.base.device.logical.destroy_descriptor_set_layout(*ubo_elem, None)
                    }
                }

                self.base.device.logical.destroy_pipeline(self.graphics_pipelines[i].graphics, None);
                self.base.device.logical.destroy_pipeline_layout(self.graphics_pipelines[i].layout, None);
            }

        }
    }
}


fn main() {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Vulkan Video")
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(window);

    let mut closing = false;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, window_id } if window_id == app.base.window.id() => {
                app.handle_event(event);
            }

            Event::AboutToWait => {
                app.base.window.request_redraw();
            }


            _ => (),
        }

        if app.close && !closing {
            closing = true;
            elwt.exit();
        }
    });

    println!("Process Completed.");
}
