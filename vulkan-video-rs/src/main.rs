mod vk_base;
mod vk_bundles;
mod shader;
mod mesh;
mod shader_utils;
mod devices;
mod drawable;
mod primitives;
mod utils;
mod rhi;
mod scene;
mod scene_extensions;
mod geometry;

use devices::record_player::RecordPlayer;
use drawable::{drawable_mesh::DrawableMesh, drawable_tex::DrawableTexture, drawable2d::Drawable2d};
use mesh::{ Rect, cube};
use primitives::texture2d::{PixelFormat, Texture2d};
use scene_extensions::simple_scene::SimpleScene;
use utils::image::{begin_single_time_command, end_single_time_command};
use vk_bundles::*;

use ash::vk;

use vk_base::VkBase;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

struct App {
    base: VkBase,
    rect_bundles: Vec<Drawable2d>,
    mesh_bundles: Vec<DrawableMesh>,
    textures: Vec<DrawableTexture>,
    scenes: Vec<SimpleScene>,
    video_device: RecordPlayer,
    close: bool
}

use shader::*;

impl App {
    fn new(window: Window) -> Self {

        ShaderRegistry::describe_registed_shaders();


        let video_device = RecordPlayer::from_buffer(include_bytes!("../assets/recordings/record1.rdbin")).unwrap();
        let base = VkBase::new(window, 3, "./assets/shaders");

        let rect_bundles = vec![
            Drawable2d::new(&base.device, Rect::new(-0.9, -0.9, 0.5, 0.5, [1.0, 0.0, 0.0])),
            Drawable2d::new(&base.device, Rect::new(0.0, 0.0, 0.5, 0.5, [0.0, 0.0, 1.0])),
            Drawable2d::new(&base.device, Rect::new(-0.25, -0.25, 0.5, 0.5, [0.0, 1.0, 1.0]))
        ];

        let mesh_bundles = vec![
            DrawableMesh::new(&base.device, cube::make_cube(0.0, 0.0, 0.25, 0.5, [1.0, 0.2, 1.0]))
        ];

        let scenes = vec![
            SimpleScene::new(&base, 2)
        ];

        let data = unsafe { video_device.current_frame[0..video_device.size() / 2].align_to::<u8>().1.to_vec() };
        let texture = Texture2d::new(data, video_device.width(), video_device.height(), PixelFormat::Z16);

        //TODO: Cleanup descriptor pool

        let cb = begin_single_time_command(&base.device, base.spare_command.pool);
        let ubo = base.graphics_pipelines[ShaderTexture::ID].ubo.as_ref().unwrap();

        let textures = vec![
            DrawableTexture::new(&base.device, base.descriptor_pool,  cb, ubo[0], base.swapchain.images.len(), Rect::new(-1.0, -1.0, 2.0, 2.0, [1.0, 1.0, 1.0]), texture)
        ];

        end_single_time_command(&base.device, base.spare_command.pool, base.device.present_queue, cb);

        Self {
            video_device,
            base,
            rect_bundles,
            mesh_bundles,
            textures,
            scenes,
            close: false
        }
    }

    fn update(&mut self) {

        for mesh_bundle in self.rect_bundles.iter_mut() {
            mesh_bundle.mesh.transform(0.001, [0.0, 0.0]);
        }

        self.base.cleanup_in_flight_buffers();

        if self.base.sync_objects.spare_fences.len() == 0 {
            return;
        }

        let cb = match self.base.spare_command.buffers.pop() {
            Some(cb) => cb,
            None => { return; }
        };

        let cb_begin_info =  vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        unsafe {
            let _ = self.base.device.logical.reset_command_buffer(cb, vk::CommandBufferResetFlags::empty());
            self.base.device.logical.begin_command_buffer(cb, &cb_begin_info).unwrap();
        }

        Drawable2d::update(&self.base.device, &cb, &mut self.rect_bundles);
        DrawableMesh::update(&self.base.device, &cb, &mut self.mesh_bundles);

        SimpleScene::update(&self.base, &cb, &mut self.scenes);


        if let Some(new_frame) = self.video_device.poll() {
            self.textures[0].texture_data.update_data(new_frame);
        }

        DrawableTexture::update(&self.base.device, cb, &mut self.textures);

        unsafe { self.base.device.logical.end_command_buffer(cb).unwrap(); }

        let cbs = [cb];
        let submit_info = vk::SubmitInfo::default()
            .command_buffers(&cbs);

        let fences = [self.base.sync_objects.spare_fences.pop().unwrap()];
        unsafe {
            self.base.device.logical.reset_fences(&fences).expect("Failed to reset fences.");
            self.base.device.logical.queue_submit(self.base.device.present_queue, &[submit_info], fences[0]).expect("Failure submitting to the queue.");
        }

        self.base.in_flight_buffers.push((cb, fences[0]));
    }

    fn render(&mut self)
    {

        let cb_data = self.base.begin_renderpass_command_buffer();
        if cb_data.is_none()
        {
            return;
        }

        let (cb, image_index) = cb_data.unwrap();

        DrawableTexture::draw(&self.base.device, cb, &self.base.graphics_pipelines[ShaderTexture::ID], self.base.current_frame, &self.textures);
        Drawable2d::draw(&self.base.device, &cb, &self.base.graphics_pipelines[ShaderRect::ID], &self.rect_bundles);
        DrawableMesh::draw(&self.base.device, &cb, &self.base.graphics_pipelines[ShaderMesh::ID], &self.mesh_bundles);
        SimpleScene::draw(&mut self.base, &cb, &self.scenes);
        self.base.render(&cb, image_index);
    }

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.close = true;
            }

            WindowEvent::RedrawRequested => {
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
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {

            let _ = self.base.device.logical.device_wait_idle();


            Drawable2d::release(&self.base.device, &mut self.rect_bundles);
            self.rect_bundles.clear();

            DrawableMesh::release(&self.base.device, &mut self.mesh_bundles);
            self.mesh_bundles.clear();

            DrawableTexture::release(&self.base.device, &mut self.textures);
            self.textures.clear();

            SimpleScene::release(&self.base, &mut self.scenes);
            self.scenes.clear();

            for i in 0..self.base.graphics_pipelines.len() {
                if let Some(ubo) = self.base.graphics_pipelines[i].ubo.as_ref() {
                    for ubo_elem in ubo {
                        self.base.device.logical.destroy_descriptor_set_layout(*ubo_elem, None)
                    }
                }

                self.base.device.logical.destroy_pipeline(self.base.graphics_pipelines[i].graphics, None);
                self.base.device.logical.destroy_pipeline_layout(self.base.graphics_pipelines[i].layout, None);
            }
        }
    }
}


fn main() {
    // SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Vulkan Video")
        .build(&event_loop)
        .unwrap();

    let mut app = App::new(window);

    let mut closing = false;

    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(winit::event_loop::ControlFlow::Poll);
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
