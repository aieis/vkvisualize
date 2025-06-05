mod vk_bundles;
mod shader;
mod mesh;
mod vk_utils;

use std::ffi::{c_char, c_void, CStr, CString};

use ash::{ext::debug_utils, khr};

use mesh::{Mesh, Vertex};
use vk_bundles::*;

use ash::vk;
use simple_logger::SimpleLogger;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::{Window, WindowBuilder},
};

const WINDOW_WIDTH: u32 = 100;
const WINDOW_HEIGHT: u32 = 100;
const MAX_FRAMES_IN_FLIGHT: usize = 10;

struct App {

    mesh_bundles: Vec<MeshBundle>,

    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    surface: SurfaceBundle,
    device: DeviceBundle,
    swapchain: SwapchainBundle,
    image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    graphics_pipeline: GraphicsPipelineBundle,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    sync_objects: SyncObjectsBundle,
    current_frame: usize,
    is_framebuffer_resized: bool,

    window: Window,
    close: bool,
}

impl App {
    fn new(window: Window) -> Self {
        let (entry, instance) = App::create_instance(&window);
        let (debug_utils_loader, debug_messenger) = App::setup_validation(&entry, &instance);

        let surface = App::create_surface(&entry, &instance, &window);
        let device = App::select_phsyical_device(&instance, &surface);
        let swapchain = App::create_swapchain(&instance, &device, &surface);
        let image_views = App::create_image_views(&device, &swapchain);
	let render_pass = App::create_render_pass(&device, &swapchain);
	let graphics_pipeline = App::create_graphics_pipeline(&device, &swapchain, &render_pass);
	let framebuffers = App::create_framebuffers(&device, &render_pass, &image_views, &swapchain);
	let command_pool = App::create_command_pool(&device);
        let mesh_bundles = App::create_vertex_objects(&instance, &device);
        let command_buffers = App::create_command_buffers(&instance, &device, command_pool, &graphics_pipeline, &framebuffers, render_pass, &swapchain, &mesh_bundles);
	let sync_objects = App::create_sync_objects(&device);


        Self {
            mesh_bundles,

            _entry: entry,

            instance,
            debug_utils_loader,
            debug_messenger,

            surface,
            device,
            swapchain,
            image_views,
	    render_pass,
	    graphics_pipeline,
	    framebuffers,
	    command_pool,
	    command_buffers,
	    sync_objects,
	    current_frame: 0,
            is_framebuffer_resized: false,

            window,
            close: false,
        }
    }

    fn render(&mut self) {
        let wait_fences = [self.sync_objects.inflight_fences[self.current_frame]];

        let (image_index, _is_sub_optimal) = unsafe {
            self.device.logical.wait_for_fences(&wait_fences, true, std::u64::MAX)
                .expect("Failed to wait for Fence!");

            let result = self.swapchain.loader.acquire_next_image(
                self.swapchain.swapchain, std::u64::MAX,
                self.sync_objects.image_available_semaphores[self.current_frame],
                vk::Fence::null());

            match result {
                Ok(image_index) => image_index,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR => {
                        self.recreate_swapchain();
                        return;
                    }
                    _ => panic!("Failed to acquire swapchain image!"),
                },
            }
        };

        let wait_semaphores = [self.sync_objects.image_available_semaphores[self.current_frame]];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let signal_semaphores = [self.sync_objects.render_finished_semaphores[self.current_frame]];

        let submit_infos = [
	    vk::SubmitInfo::default()
		.wait_semaphores(&wait_semaphores)
		.wait_dst_stage_mask(&wait_stages)
		.command_buffers(&self.command_buffers[image_index as usize..image_index as usize + 1])
		.signal_semaphores(&signal_semaphores)
        ];

        unsafe {
            self.device.logical.reset_fences(&wait_fences).expect("Failed to reset Fence!");
	    self.device.logical.queue_submit(self.device.present_queue, &submit_infos, self.sync_objects.inflight_fences[self.current_frame],)
                .expect("Failed to execute queue submit.");
        }

        let swapchains = [self.swapchain.swapchain];

	let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
	    .wait_semaphores(&signal_semaphores)
	    .swapchains(&swapchains)
            .image_indices(&image_indices);

        let result =  unsafe { self.swapchain.loader.queue_present(self.device.present_queue, &present_info) };

        let is_resized = match result {
            Ok(_) => self.is_framebuffer_resized,
            Err(vk_result) => match vk_result {
                vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                _ => panic!("Failed to execute queue present."),
            },
        };

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        if is_resized {
            self.is_framebuffer_resized = false;
            self.recreate_swapchain();
            return self.render();
        }
    }

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.close = true;
            }
            WindowEvent::RedrawRequested => {
                self.window.pre_present_notify();
                self.render();
            }

            WindowEvent::Resized(_) => {
                self.window.request_redraw();
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

    fn recreate_swapchain(&mut self) {
        // parameters -------------
        unsafe { self.device.logical.device_wait_idle().expect("Failed to wait device idle!") };
        self.cleanup_swapchain();

        self.swapchain = App::create_swapchain(&self.instance, &self.device, &self.surface);
        self.image_views = App::create_image_views(&self.device, &self.swapchain);
	self.render_pass = App::create_render_pass(&self.device, &self.swapchain);
	self.graphics_pipeline = App::create_graphics_pipeline(&self.device, &self.swapchain, &self.render_pass);
	self.framebuffers = App::create_framebuffers(&self.device, &self.render_pass, &self.image_views, &self.swapchain);
        self.command_buffers = App::create_command_buffers(&self.instance, &self.device, self.command_pool, &self.graphics_pipeline, &self.framebuffers, self.render_pass, &self.swapchain, &self.mesh_bundles);
    }

    fn cleanup_swapchain(&self) {
        unsafe {
            self.device.logical.free_command_buffers(self.command_pool, &self.command_buffers);
            for &framebuffer in self.framebuffers.iter() {
                self.device.logical.destroy_framebuffer(framebuffer, None);
            }
            self.device.logical.destroy_pipeline(self.graphics_pipeline.graphics, None);
            self.device.logical.destroy_pipeline_layout(self.graphics_pipeline.layout, None);
            self.device.logical.destroy_render_pass(self.render_pass, None);
            for &image_view in self.image_views.iter() {
                self.device.logical.destroy_image_view(image_view, None);
            }
            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
        }
    }


    /* Misc vulkan */


    fn create_instance(window: &Window) -> (ash::Entry, ash::Instance) {
        // The entry contains the global vk functions
        let entry = unsafe { ash::Entry::load().unwrap() };

        // List all the supported layers
        let vk_layers = unsafe {entry.enumerate_instance_layer_properties().expect("Could not enumerate layers")};

        println!("Found {} layer(s).", vk_layers.len());
        for layer in vk_layers {
            println!("\t{:?}", layer.layer_name_as_c_str().unwrap())
        }
        println!();

        // Select layers to enable
        let layers = [c"VK_LAYER_KHRONOS_validation"];
        let layers_raw: Vec<*const c_char> = layers.iter().map(|raw_name| raw_name.as_ptr()).collect();

        // List all the supported extensions
        let vk_extensions = unsafe { entry.enumerate_instance_extension_properties(None).unwrap() };
        println!("Found {} extension(s).", vk_extensions.len());
        for extension in vk_extensions {
            println!("\t{:?}", extension.extension_name_as_c_str().unwrap())
        }
        println!();

        // Select the extensions
        let mut extensions = ash_window::enumerate_required_extensions(window.display_handle().unwrap().as_raw()).unwrap().to_vec();
        extensions.push(debug_utils::NAME.as_ptr());

        // Create the instance
        let app_name = c"Vulkan Video";

        let app_info = vk::ApplicationInfo::default()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(vk::make_api_version(0, 1, 0, 0));

        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&layers_raw)
            .enabled_extension_names(&extensions);

        let instance = unsafe { entry.create_instance(&create_info, None).expect("Failed to create instance.") };

        (entry, instance)
    }

    fn create_surface(entry: &ash::Entry, instance: &ash::Instance, window: &Window) -> SurfaceBundle {
        let display_handle = window.display_handle().unwrap();
        let window_handle = window.window_handle().unwrap();
        let surface = unsafe { ash_window::create_surface(entry, instance, display_handle.as_raw(), window_handle.as_raw(), None).unwrap() };
        let surface_loader = khr::surface::Instance::new(entry, instance);

        SurfaceBundle {
            surface,
            loader: surface_loader
        }
    }

    /* Select device */
    fn select_phsyical_device(instance: &ash::Instance, surface: &SurfaceBundle) -> DeviceBundle{
        let devs = unsafe { instance.enumerate_physical_devices().unwrap() };

        let mut queues = Vec::new();

        println!("Found {} device(s).", devs.len());
        for dev in devs.iter() {
            let properties = unsafe { instance.get_physical_device_properties(*dev) };
            let queue_props = unsafe { instance.get_physical_device_queue_family_properties(*dev) };

            for (i, queue) in queue_props.iter().enumerate() {
                let surface_support = unsafe { surface.loader.get_physical_device_surface_support(*dev, i as u32, surface.surface).unwrap() };
                if surface_support && queue.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    queues.push((i as u32, *dev));
                }
            }

            println!("\t{:?}", properties.device_name_as_c_str().unwrap());
        }

        println!();

        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queues[0].0)
            .queue_priorities(&[1.0]);

        let device_extension_names_raw = [
            khr::swapchain::NAME.as_ptr(),
        ];

        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_extension_names(&device_extension_names_raw);

        let device: ash::Device = unsafe { instance.create_device(queues[0].1, &device_create_info, None).unwrap() };
        let present_queue = unsafe { device.get_device_queue(queues[0].0 as  u32, 0) };

        DeviceBundle {
            logical: device,
            physical: queues[0].1,
            queue_family_index: queues[0].0 as u32,
            present_queue,
        }
    }

    /* Setup the swapchain */
    fn create_swapchain(instance: &ash::Instance, device: &DeviceBundle, surface: &SurfaceBundle) -> SwapchainBundle {

        let surface_format = unsafe { surface.loader.get_physical_device_surface_formats(device.physical, surface.surface).unwrap()[0] };

        let surface_capabilities = unsafe { surface.loader.get_physical_device_surface_capabilities(device.physical, surface.surface).unwrap() };
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count
        {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let surface_resolution = match surface_capabilities.current_extent.width {
            u32::MAX => vk::Extent2D {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
            },
            _ => surface_capabilities.current_extent,
        };

        let pre_transform = if surface_capabilities.supported_transforms.contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
        {
            vk::SurfaceTransformFlagsKHR::IDENTITY
        } else {
            surface_capabilities.current_transform
        };

        let present_modes = unsafe { surface.loader.get_physical_device_surface_present_modes(device.physical, surface.surface).unwrap() };

        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        let swapchain_loader = khr::swapchain::Device::new(&instance, &device.logical);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .surface(surface.surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(surface_resolution)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(pre_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .image_array_layers(1);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap() };
        let present_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

        SwapchainBundle {
            swapchain,
            loader: swapchain_loader,
            images: present_images,
            format: surface_format.format,
            extent: surface_resolution
        }
    }

    fn create_image_views(device: &DeviceBundle, swapchain: &SwapchainBundle) -> Vec<vk::ImageView>{
        let mut present_image_views: Vec<vk::ImageView> = Vec::new();
        let rgba_component = vk::ComponentMapping {
            r: vk::ComponentSwizzle::R,
            g: vk::ComponentSwizzle::G,
            b: vk::ComponentSwizzle::B,
            a: vk::ComponentSwizzle::A,
        };

        let subresource_range = vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        };

        for image in swapchain.images.iter() {
            let create_view_info = vk::ImageViewCreateInfo::default()
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain.format)
                .components(rgba_component)
                .subresource_range(subresource_range)
                .image(*image);
            let view = unsafe { device.logical.create_image_view(&create_view_info, None).unwrap() };
            present_image_views.push(view);
        }

        present_image_views
    }

    /* Setup the graphics pipeline */
    fn create_graphics_pipeline(device: &DeviceBundle, swapchain: &SwapchainBundle, renderpass: &vk::RenderPass) -> GraphicsPipelineBundle {
	let shader = shader::Shader::new(&device.logical, "assets/shaders/triangle.vert.spv", "assets/shaders/triangle.frag.spv");
        let main_function_name = CString::new("main").unwrap(); // the beginning function name in shader code.
	let vertex_shader_stage = vk::PipelineShaderStageCreateInfo::default()
	    .name(&main_function_name)
	    .stage(vk::ShaderStageFlags::VERTEX)
	    .module(shader.vertex);

	let fragment_shader_stage = vk::PipelineShaderStageCreateInfo::default()
	    .name(&main_function_name)
	    .stage(vk::ShaderStageFlags::FRAGMENT)
	    .module(shader.fragment);

	let shader_stage_create_infos = [vertex_shader_stage, fragment_shader_stage];

        let bindings = Mesh::get_binding_descriptions();
        let attributes = Mesh::get_attribute_descriptions();
        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);

	let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo::default()
	    .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
	    .primitive_restart_enable(false);

	let viewports = [vk::Viewport {
	    x: 0.0,
	    y: 0.0,
	    width: swapchain.extent.width as f32,
	    height: swapchain.extent.height as f32,
	    min_depth: 0.0,
	    max_depth: 1.0
	}];

	let scissors = [vk::Rect2D {
	    offset: vk::Offset2D {x: 0, y: 0},
	    extent: swapchain.extent
	}];

	// let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
	let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default();
	//.dynamic_states(&dynamic_states);

	let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
	    .viewports(&viewports)
	    .scissors(&scissors);

	let rasterization_info = vk::PipelineRasterizationStateCreateInfo::default()
	    .cull_mode(vk::CullModeFlags::BACK)
	    .front_face(vk::FrontFace::CLOCKWISE)
	    .polygon_mode(vk::PolygonMode::FILL)
	    .depth_clamp_enable(false)
	    .rasterizer_discard_enable(false)
	    .line_width(1.0);

	let multisample_state_info = vk::PipelineMultisampleStateCreateInfo::default()
	    .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };

        let depth_state_create_info = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false)
            .front(stencil_state)
            .back(stencil_state)
            .max_depth_bounds(1.0)
            .min_depth_bounds(0.0);


        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        }];

	let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

	let layout_create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe { device.logical.create_pipeline_layout(&layout_create_info, None).unwrap() };

        let graphic_pipeline_infos = [vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_create_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(*renderpass)];

        let graphics_pipelines = unsafe {
            device.logical.create_graphics_pipelines(vk::PipelineCache::null(), &graphic_pipeline_infos, None)
                .expect("Failed to create Graphics Pipeline!.")
        };

        unsafe {
            device.logical.destroy_shader_module(shader.vertex, None);
            device.logical.destroy_shader_module(shader.fragment, None);
        }


	GraphicsPipelineBundle {
	    graphics: graphics_pipelines[0],
	    layout: pipeline_layout
	}
    }

    fn create_render_pass(device: &DeviceBundle, swapchain: &SwapchainBundle) -> vk::RenderPass{
        let color_attachment = vk::AttachmentDescription::default()
            .format(swapchain.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        let subpass = [vk::SubpassDescription::default()
	    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
	    .color_attachments(&color_attachment_ref)];

        let render_pass_attachments = [color_attachment];

        let renderpass_create_info = vk::RenderPassCreateInfo::default()
	    .attachments(&render_pass_attachments)
	    .subpasses(&subpass);

        unsafe {
            device.logical.create_render_pass(&renderpass_create_info, None)
                .expect("Failed to create render pass!")
        }
    }

    fn create_framebuffers(device: &DeviceBundle, render_pass: &vk::RenderPass, image_views: &Vec<vk::ImageView>, swapchain: &SwapchainBundle) -> Vec<vk::Framebuffer>{
	let mut framebuffers = vec![];

	for &image_view in image_views.iter() {
            let attachments = [image_view];

            let framebuffer_create_info = vk::FramebufferCreateInfo::default()
                .render_pass(*render_pass)
		.attachments(&attachments)
		.width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1);

            let framebuffer = unsafe {
                device.logical.create_framebuffer(&framebuffer_create_info, None)
                    .expect("Failed to create Framebuffer!")
            };

            framebuffers.push(framebuffer);
	}

	framebuffers
    }

    fn create_command_pool(device: &DeviceBundle) -> vk::CommandPool {
        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
	    .queue_family_index(device.queue_family_index);

        unsafe {
            device.logical.create_command_pool(&command_pool_create_info, None)
                .expect("Failed to create Command Pool!")
        }
    }

    fn create_vertex_objects(instance: &ash::Instance, device: &DeviceBundle) -> Vec<MeshBundle>{
        let triangle = Mesh::triangle();
        let (vertex_buffer, vertex_buffer_memory) = vk_utils::gen_vertex_buffer(instance, device, triangle.size() as u64).expect("Failed to create vertex buffer.");
        unsafe {
            device.logical.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0).expect("Failed to bind buffer");
            let data_ptr = device.logical.map_memory(vertex_buffer_memory, 0, triangle.size() as u64, vk::MemoryMapFlags::empty())
                .expect("Failed to map memory") as *mut Vertex;
            data_ptr.copy_from_nonoverlapping(triangle.vertices.as_ptr(), triangle.vertices.len());
            device.logical.unmap_memory(vertex_buffer_memory);
        }

        return vec![MeshBundle { mesh: triangle, vbo: vertex_buffer, vbo_mem: vertex_buffer_memory}];
    }

    fn create_command_buffers(instance: &ash::Instance, device: &DeviceBundle, command_pool: vk::CommandPool, graphics_pipeline: &GraphicsPipelineBundle, framebuffers: &Vec<vk::Framebuffer>, render_pass: vk::RenderPass, swapchain: &SwapchainBundle, mesh_bundles: &[MeshBundle]) -> Vec<vk::CommandBuffer> {
        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
	    .command_buffer_count(framebuffers.len() as u32)
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY);

        let command_buffers = unsafe {
            device.logical.allocate_command_buffers(&command_buffer_allocate_info)
                .expect("Failed to allocate Command Buffers!")
        };

        for (i, &command_buffer) in command_buffers.iter().enumerate() {
            let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
		.flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

            unsafe {
                device.logical.begin_command_buffer(command_buffer, &command_buffer_begin_info)
                    .expect("Failed to begin recording Command Buffer at beginning!");
            }

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = vk::RenderPassBeginInfo::default()
                .render_pass(render_pass)
                .framebuffer(framebuffers[i])
                .render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: swapchain.extent})
                .clear_values(&clear_values);

            unsafe {
                device.logical.cmd_begin_render_pass(command_buffer, &render_pass_begin_info, vk::SubpassContents::INLINE);
                device.logical.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, graphics_pipeline.graphics);
                device.logical.cmd_bind_vertex_buffers(command_buffer, 0, &[mesh_bundles[0].vbo], &[0]);
                device.logical.cmd_draw(command_buffer, mesh_bundles[0].mesh.vertices.len() as u32, 1, 0, 0);
                device.logical.cmd_end_render_pass(command_buffer);
		device.logical.end_command_buffer(command_buffer).expect("Failed to record Command Buffer at Ending!");
            }
        }

        command_buffers
    }

    fn create_sync_objects(device: &DeviceBundle) -> SyncObjectsBundle {
        let mut sync_objects = SyncObjectsBundle {
            image_available_semaphores: vec![],
            render_finished_semaphores: vec![],
            inflight_fences: vec![],
        };

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();

        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                let image_available_semaphore = device.logical.create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let render_finished_semaphore = device.logical.create_semaphore(&semaphore_create_info, None)
                    .expect("Failed to create Semaphore Object!");
                let inflight_fence = device.logical.create_fence(&fence_create_info, None)
                    .expect("Failed to create Fence Object!");

                sync_objects.image_available_semaphores.push(image_available_semaphore);
                sync_objects.render_finished_semaphores.push(render_finished_semaphore);
                sync_objects.inflight_fences.push(inflight_fence);
            }
        }

        sync_objects
    }

    /* Setup validation layer callbacks */
    fn setup_validation(entry: &ash::Entry, instance: &ash::Instance) -> (debug_utils::Instance, vk::DebugUtilsMessengerEXT) {
        let message_severity = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            // | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            // | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;

        let message_type = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION;

        let messenger_ci = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(message_severity)
            .message_type(message_type)
            .pfn_user_callback(Some(vulkan_debug_utils_callback));

        let debug_utils_loader = debug_utils::Instance::new(entry, instance);
        let utils_messenger = unsafe {
            debug_utils_loader.create_debug_utils_messenger(&messenger_ci, None).expect("Debug Utils Callback")
        };
        (debug_utils_loader, utils_messenger)
    }

}


impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.logical.device_wait_idle();

            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device.logical.destroy_semaphore(self.sync_objects.image_available_semaphores[i], None);
                self.device.logical.destroy_semaphore(self.sync_objects.render_finished_semaphores[i], None);
                self.device.logical.destroy_fence(self.sync_objects.inflight_fences[i], None);
            }

            self.cleanup_swapchain();

            for mesh in self.mesh_bundles.iter() {
                self.device.logical.destroy_buffer(mesh.vbo, None);
                self.device.logical.free_memory(mesh.vbo_mem, None);
            }

	    self.device.logical.destroy_command_pool(self.command_pool, None);

            self.device.logical.destroy_device(None);
            self.surface.loader.destroy_surface(self.surface.surface, None);

            self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_messenger, None);
            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    vk::FALSE
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
            Event::WindowEvent { event, window_id } if window_id == app.window.id() => {
                app.handle_event(event);
            }

            Event::AboutToWait => {
                app.window.request_redraw();
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
