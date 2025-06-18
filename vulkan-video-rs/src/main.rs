mod vk_base;
mod vk_bundles;
mod shader;
mod mesh;
mod vk_utils;
mod shader_utils;
mod devices;

use mesh::{Rect, Vertex};
use vk_bundles::*;
use vk_utils::*;

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
    mesh_bundles: Vec<MeshBundle>,
    graphics_pipelines: Vec<GraphicsPipelineBundle>,
    close: bool,
}

impl App {
    fn new(window: Window) -> Self {
        let base = VkBase::new(window, 3);

        let mesh_bundles = vec![
            create_mesh_bundle(&base.device, Rect::new(-0.9, -0.9, 0.5, 0.5, [1.0, 0.0, 0.0])),
            create_mesh_bundle(&base.device, Rect::new(0.0, 0.0, 0.5, 0.5, [0.0, 0.0, 1.0])),
        ];

        let graphics_pipelines = vec![base.create_graphics_pipeline(Box::from(make_shader!("triangle")))];

        Self {
            base,
            mesh_bundles,
            graphics_pipelines,
            close: false
        }
    }

    fn update(&mut self) {

        self.base.cleanup_in_flight_buffers();

        if self.base.sync_objects.spare_fences.len() == 0 {
            return;
        }

        let command_buffer = self.base.spare_command.buffers.pop();
        if command_buffer.is_none() {
            return;
        }

        let command_buffers = [command_buffer.unwrap()];

        for mesh_bundle in self.mesh_bundles.iter_mut() {
            unsafe {
                let data_ptr = self.base.device.logical.map_memory(mesh_bundle.staging.memory, 0, mesh_bundle.mesh.size_vrt() as u64, vk::MemoryMapFlags::empty())
                    .expect("Failed to map memory") as *mut Vertex;
                data_ptr.copy_from_nonoverlapping(mesh_bundle.mesh.vertices.as_ptr(), mesh_bundle.mesh.vertices.len());
                self.base.device.logical.unmap_memory(mesh_bundle.staging.memory);

                let data_ptr = self.base.device.logical.map_memory(mesh_bundle.staging_ind.memory, 0, mesh_bundle.mesh.size_ind() as u64, vk::MemoryMapFlags::empty())
                    .expect("Failed to map memory") as *mut u16;
                data_ptr.copy_from_nonoverlapping(mesh_bundle.mesh.indices.as_ptr(), mesh_bundle.mesh.indices.len());
                self.base.device.logical.unmap_memory(mesh_bundle.staging_ind.memory);
            }
        }

        vk_utils::record_mesh_update(&self.base.device, &command_buffers[0], &self.mesh_bundles);

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
        self.base.record_command_buffer(&command_buffers[0], &self.graphics_pipelines[0], &self.mesh_bundles);
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
        self.base.recreate_swapchain();

        for i in 0..self.graphics_pipelines.len() {
            unsafe {
                self.base.device.logical.destroy_pipeline(self.graphics_pipelines[i].graphics, None);
                self.base.device.logical.destroy_pipeline_layout(self.graphics_pipelines[i].layout, None);
            }

            self.base.recreate_graphics_pipeline(&mut self.graphics_pipelines[i]);
        }
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
                self.base.device.logical.destroy_buffer(mesh.staging_ind.buffer, None);
                self.base.device.logical.free_memory(mesh.staging_ind.memory, None);
                self.base.device.logical.destroy_buffer(mesh.ind.buffer, None);
                self.base.device.logical.free_memory(mesh.ind.memory, None);
            }

            for i in 0..self.graphics_pipelines.len() {
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
