mod vk_bundles;
mod shader;

use std::ffi::{c_char, c_void, CStr, CString};

use ash::{ext::debug_utils, khr};

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

const WINDOW_WIDTH: u32 = 1080;
const WINDOW_HEIGHT: u32 = 1080;

struct App {
    entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: debug_utils::Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,

    surface: SurfaceBundle,
    device: DeviceBundle,
    swapchain: SwapchainBundle,
    image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    graphics_pipeline: GraphicsPipelineBundle,

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

        Self {
            entry,
            instance,
            debug_utils_loader,
            debug_messenger,

            surface,
            device,
            swapchain,
            image_views,
	    render_pass,
	    graphics_pipeline,

            window,
            close: false,
        }
    }

    fn render(&self) {}

    fn handle_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.close = true;
            }
            WindowEvent::RedrawRequested => {
                self.window.pre_present_notify();
                self.render();
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
        let app_name = CString::new("Vulkan Video").unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();

        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::make_api_version(1, 0, 1, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: layers_raw.as_ptr(),
            enabled_layer_count: layers.len() as u32,
            pp_enabled_extension_names: extensions.as_ptr(),
            enabled_extension_count: extensions.len() as u32,
            ..Default::default()
        };

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

        DeviceBundle {
            logical: device,
            physical: queues[0].1,
            queue_family_index: queues[0].0 as u32
        }
    }

    /* Setup the swapchain */
    fn create_swapchain(
        instance: &ash::Instance,
        device: &DeviceBundle,
        surface: &SurfaceBundle
    ) -> SwapchainBundle {

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

	let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::default();
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

	let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
	let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::default()
	    .dynamic_states(&dynamic_states);

	let viewport_state_info = vk::PipelineViewportStateCreateInfo::default()
	    .viewports(&viewports)
	    .scissors(&scissors);

	let rasterization_info = vk::PipelineRasterizationStateCreateInfo::default()
	    .cull_mode(vk::CullModeFlags::FRONT)
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
    }}


impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            for &image in self.image_views.iter() {
                self.device.logical.destroy_image_view(image, None);
            }

            self.swapchain.loader.destroy_swapchain(self.swapchain.swapchain, None);
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
